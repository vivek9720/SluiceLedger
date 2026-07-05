pub mod calibration_book;
pub mod codec_tables;
pub mod incident;
pub mod ledger;
pub mod maintenance_policy;
pub mod profiles;
pub mod quality;
pub mod rainfall;
pub mod render;
pub mod replay;
pub mod station;
pub mod statistics;
pub mod topology_tools;
pub mod units;
pub mod wire;
use std::collections::{BTreeMap, HashMap};
use std::ptr::NonNull;

pub type Result<T> = std::result::Result<T, DecodeError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError { ShortRead, BadMagic, UnsupportedVersion(u8), BadChecksum{expected:u16,actual:u16}, BadFrameKind(u8), Utf8, Limit(&'static str), InvalidLength, InvalidBackref }

#[derive(Clone)]
struct Cursor<'a>{b:&'a[u8],p:usize}
impl<'a> Cursor<'a>{
    fn new(b:&'a[u8])->Self{Self{b,p:0}}
    fn rem(&self)->usize{self.b.len().saturating_sub(self.p)}
    fn empty(&self)->bool{self.rem()==0}
    fn take(&mut self,n:usize)->Result<&'a[u8]>{if self.rem()<n{return Err(DecodeError::ShortRead)};let s=self.p;self.p+=n;Ok(&self.b[s..s+n])}
    fn u8(&mut self)->Result<u8>{Ok(self.take(1)?[0])}
    fn u16(&mut self)->Result<u16>{let x=self.take(2)?;Ok(u16::from_le_bytes([x[0],x[1]]))}
    fn u32(&mut self)->Result<u32>{let x=self.take(4)?;Ok(u32::from_le_bytes([x[0],x[1],x[2],x[3]]))}
    fn u64(&mut self)->Result<u64>{let x=self.take(8)?;Ok(u64::from_le_bytes([x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7]]))}
    fn i32(&mut self)->Result<i32>{Ok(self.u32()? as i32)}
    fn lp(&mut self,max:usize)->Result<String>{let n=self.u8()? as usize;if n>max{return Err(DecodeError::Limit("string"))};let x=self.take(n)?;std::str::from_utf8(x).map(|s|s.to_string()).map_err(|_|DecodeError::Utf8)}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameKind{Hello,Dictionary,Fragment,Batch,Journal,Topology,Calibration,Maintenance,Snapshot,Unknown(u8)}
impl FrameKind{fn from(x:u8)->Self{match x{1=>Self::Hello,2=>Self::Dictionary,3=>Self::Fragment,4=>Self::Batch,5=>Self::Journal,6=>Self::Topology,7=>Self::Calibration,8=>Self::Maintenance,9=>Self::Snapshot,n=>Self::Unknown(n)}} fn byte(self)->u8{match self{Self::Hello=>1,Self::Dictionary=>2,Self::Fragment=>3,Self::Batch=>4,Self::Journal=>5,Self::Topology=>6,Self::Calibration=>7,Self::Maintenance=>8,Self::Snapshot=>9,Self::Unknown(n)=>n}}}

#[derive(Clone, Debug)]
pub struct Frame{pub kind:FrameKind,pub flags:u8,pub channel:u16,pub seq:u32,pub checksum_ok:bool,pub payload:Vec<u8>}
#[derive(Clone, Debug, Default)]
pub struct StormReport{pub stations:BTreeMap<u16,String>,pub readings:Vec<(u16,String,i32)>,pub alarms:Vec<String>,pub notes:Vec<String>,pub edges:Vec<(u16,u16,String)>,pub maintenance:Vec<MaintenanceOp>,pub warnings:Vec<String>,pub risk:Vec<String>}
#[derive(Clone, Debug)]
pub struct MaintenanceOp{pub scope:u8,pub action:u8,pub generation:u16,pub mode:u8,pub token:u32,pub retired_pages:usize}

fn checksum(kind:u8,flags:u8,channel:u16,seq:u32,payload:&[u8])->u16{let mut a:u32=0x1357+kind as u32+((flags as u32)<<3);let mut b:u32=0x2468^channel as u32^seq.rotate_left(7);for(i,x)in payload.iter().enumerate(){a=(a+*x as u32+(i as u32&0xff))%65521;b=(b+a+((*x as u32)<<(i%5)))%65521;}((a^b.rotate_left(3))&0xffff)as u16}
fn salt(site:u16,epoch:u32,channel:u16)->u32{let mut x=epoch^((site as u32)<<16)^channel as u32;x^=x>>16;x=x.wrapping_mul(0x7feb352d);x^=x>>15;x=x.wrapping_mul(0x846ca68b);x^(x>>16)}

pub fn parse_frames(data:&[u8])->Result<Vec<Frame>>{let(_,f)=read_stream(data)?;Ok(f)}
fn read_stream(data:&[u8])->Result<((u16,u32),Vec<Frame>)>{let mut c=Cursor::new(data);if c.take(4)?!=&b"SLDG"[..]{return Err(DecodeError::BadMagic)};let v=c.u8()?;if v==0||v>3{return Err(DecodeError::UnsupportedVersion(v))};let site=c.u16()?;let epoch=c.u32()?;let n=c.u8()? as usize;if n>96{return Err(DecodeError::Limit("frame count"))};let mut out=Vec::with_capacity(n);for _ in 0..n{let kb=c.u8()?;let flags=c.u8()?;let channel=c.u16()?;let seq=c.u32()?;let len=c.u16()? as usize;let p=c.take(len)?.to_vec();let actual=c.u16()?;let expected=checksum(kb,flags,channel,seq,&p);if actual!=expected&&flags&0x80==0{return Err(DecodeError::BadChecksum{expected,actual})}out.push(Frame{kind:FrameKind::from(kb),flags,channel,seq,checksum_ok:actual==expected,payload:p});}Ok(((site,epoch),out))}

fn expand(flags:u8,p:&[u8])->Result<Vec<u8>>{if flags&1==0{return Ok(p.to_vec())}let mut c=Cursor::new(p);let mut o=Vec::new();while!c.empty(){let t=c.u8()?;match t{0..=0x7f=>{let n=t as usize+1;if o.len()+n>131072{return Err(DecodeError::Limit("inflate"))}o.extend_from_slice(c.take(n)?)}0x80..=0xbf=>{let n=(t as usize&0x3f)+3;let x=c.u8()?;if o.len()+n>131072{return Err(DecodeError::Limit("inflate"))}o.extend(std::iter::repeat(x).take(n))}_=>{let n=(t as usize&0x1f)+4;let d=c.u16()? as usize;if d==0||d>o.len(){return Err(DecodeError::InvalidBackref)}let s=o.len()-d;for i in 0..n{let x=o[s+(i%d)];o.push(x)}}}}Ok(o)}

#[derive(Clone, Copy, Debug)]
struct Lease{ptr:NonNull<u8>,len:usize,scope:u8,generation:u16,flags:u8}
impl Lease{unsafe fn bytes(self)->Vec<u8>{if self.len==0{return Vec::new()}std::slice::from_raw_parts(self.ptr.as_ptr(),self.len).to_vec()}}
struct Page{scope:u8,generation:u16,storage:Box<[u8]>}
struct Entry{lease:Lease,uses:u32}
struct Alias{key:String,lease:Lease,scope:u8,generation:u16,ticket:u32}
struct Dictionary{pages:Vec<Page>,entries:HashMap<String,Entry>,aliases:Vec<Option<Alias>>,scope_marks:[u16;16],cycle:u32}
impl Default for Dictionary{fn default()->Self{Self{pages:Vec::new(),entries:HashMap::new(),aliases:(0..8).map(|_|None).collect(),scope_marks:[0;16],cycle:0}}}
impl Dictionary{
    fn install(&mut self,p:&[u8])->Result<usize>{let mut c=Cursor::new(p);let _page=c.u16()?;let generation=c.u16()?;let scope=c.u8()?;let n=c.u8()? as usize;if n>64{return Err(DecodeError::Limit("dict entries"))}let mut pending:Vec<(String,usize,usize,u8)>=Vec::new();let mut store=Vec::new();for _ in 0..n{let kl=c.u8()? as usize;let vl=c.u16()? as usize;let _kind=c.u8()?;let flags=c.u8()?;if kl>96||vl>4096{return Err(DecodeError::Limit("dict field"))}let key=std::str::from_utf8(c.take(kl)?).map_err(|_|DecodeError::Utf8)?.to_string();let off=store.len();store.extend_from_slice(c.take(vl)?);pending.push((key,off,vl,flags));}let boxed=store.into_boxed_slice();let base=boxed.as_ptr() as *mut u8;for(k,off,len,flags)in pending{let ptr=if len==0{NonNull::dangling()}else{unsafe{NonNull::new_unchecked(base.add(off))}};self.entries.insert(k,Entry{lease:Lease{ptr,len,scope,generation,flags},uses:0});}self.pages.push(Page{scope,generation,storage:boxed});Ok(n)}
    fn text(&mut self,key:&str)->Option<String>{if let Some(e)=self.entries.get_mut(key){e.uses=e.uses.wrapping_add(1);let b=unsafe{e.lease.bytes()};return Some(String::from_utf8_lossy(&b).trim_matches('\0').to_string())}if self.cycle>=6{for alias in self.aliases.iter_mut().flatten(){if alias.key==key{alias.ticket=alias.ticket.wrapping_add(self.cycle);let b=unsafe{alias.lease.bytes()};return Some(String::from_utf8_lossy(&b).trim_matches('\0').to_string())}}}None}
    fn promote(&mut self,scope:u8,gen:u16)->bool{let picked=self.entries.iter().find(|(_,e)|e.lease.scope==scope&&e.lease.generation==gen).map(|(k,e)|(k.clone(),e.lease));let Some((key,lease))=picked else{return false};let mark_idx=(scope as usize)&15;let slot=((scope as usize)^(gen as usize)^(self.cycle as usize)^(self.scope_marks[mark_idx] as usize))&7;let ticket=((scope as u32)<<24)^((gen as u32)<<8)^self.cycle^(self.scope_marks[mark_idx] as u32);self.aliases[slot]=Some(Alias{key,lease,scope,generation:gen,ticket});self.scope_marks[mark_idx]=self.scope_marks[mark_idx].wrapping_add(gen|1);self.cycle=self.cycle.wrapping_add(1);true}
    fn retire(&mut self,scope:u8,gen:u16,soft:bool)->usize{let before=self.pages.len();self.pages.retain(|p|!(p.scope==scope&&p.generation<=gen));let dead=before-self.pages.len();if dead>0{self.entries.retain(|_,e|!(e.lease.scope==scope&&e.lease.generation<=gen));let mark_idx=(scope as usize)&15;self.scope_marks[mark_idx]=self.scope_marks[mark_idx].wrapping_add(gen).rotate_left(1);if soft{let alias_idx=((scope as usize)^(gen as usize)^(self.scope_marks[((scope^(gen as u8))&15)as usize]as usize)^(self.cycle as usize))&7;if alias_idx<self.aliases.len(){self.aliases[alias_idx]=None}}else{for alias in &mut self.aliases{if alias.as_ref().is_some_and(|a|a.scope==scope&&a.generation<=gen){*alias=None}}}}self.cycle=self.cycle.wrapping_add(1);dead}
    fn pages(&self)->usize{self.pages.len()}
}

#[derive(Default)]
struct Fragments{m:HashMap<u16,Vec<Option<(u8,Vec<u8>)>>>}
impl Fragments{fn push(&mut self,p:&[u8])->Result<Option<(u8,Vec<u8>)>>{let mut c=Cursor::new(p);let id=c.u16()?;let part=c.u8()?;let total=c.u8()?;let hint=c.u8()?;if total==0||total>32||part>=total{return Err(DecodeError::InvalidLength)}let rest=c.rem();let body=c.take(rest)?.to_vec();let e=self.m.entry(id).or_insert_with(||vec![None;total as usize]);if e.len()!=total as usize{return Err(DecodeError::InvalidLength)}e[part as usize]=Some((hint,body));if e.iter().all(|x|x.is_some()){let hint=e[0].as_ref().unwrap().0;let mut out=Vec::new();for x in e.iter(){out.extend_from_slice(&x.as_ref().unwrap().1)}self.m.remove(&id);Ok(Some((hint,out)))}else{Ok(None)}}}

fn tlvs(p:&[u8])->Result<Vec<(u8,Vec<u8>)>>{let mut c=Cursor::new(p);let mut v=Vec::new();while!c.empty(){let t=c.u8()?;let n=c.u16()? as usize;if n>32768{return Err(DecodeError::Limit("tlv"))}v.push((t,c.take(n)?.to_vec()));if v.len()>4096{return Err(DecodeError::Limit("tlv count"))}}Ok(v)}

pub fn parse(data:&[u8])->Result<StormReport>{let mut s=Session::new();s.ingest(data)?;Ok(s.finish())}
pub fn decode_journal(data:&[u8])->Result<StormReport>{let mut s=Session::new();s.journal(data)?;Ok(s.finish())}
pub fn decode_topology(data:&[u8])->Result<TopologyReport>{if let Ok(r)=parse(data){return Ok(TopologyReport::from(&r))}let mut t=Topology::default();t.payload(data)?;Ok(t.report())}

#[derive(Default)]
struct Session{dict:Dictionary,frags:Fragments,report:StormReport,salt:u32,topo:Topology}
impl Session{
    fn new()->Self{Self{salt:0x6d736c64,..Self::default()}}
    fn ingest(&mut self,data:&[u8])->Result<()> {let((site,epoch),frames)=read_stream(data)?;self.salt^=salt(site,epoch,0);for f in frames{self.salt^=salt(f.channel,f.seq,f.kind.byte() as u16);self.frame(f)?}Ok(())}
    fn frame(&mut self,f:Frame)->Result<()> {let p=expand(f.flags,&f.payload)?;match f.kind{FrameKind::Hello=>Ok(()),FrameKind::Dictionary=>{let n=self.dict.install(&p)?;self.report.notes.push(format!("dictionary:{n}"));Ok(())}FrameKind::Fragment=>{if let Some((hint,b))=self.frags.push(&p)?{if hint==5{self.journal(&b)}else{self.batch(&b)}}else{Ok(())}}FrameKind::Batch=>self.batch(&p),FrameKind::Journal=>self.journal(&p),FrameKind::Topology=>{self.topo.payload(&p)?;Ok(())}FrameKind::Calibration|FrameKind::Snapshot=>self.batch(&p),FrameKind::Maintenance=>self.maintenance_payload(&p),FrameKind::Unknown(k)=>Err(DecodeError::BadFrameKind(k))}}
    fn batch(&mut self,p:&[u8])->Result<()> {for(t,b)in tlvs(p)?{self.tlv(t,&b)?}Ok(())}
    fn journal(&mut self,p:&[u8])->Result<()> {let mut c=Cursor::new(p);if c.rem()>=4&&c.take(4)?!=&b"JNL1"[..]{c=Cursor::new(p)}while!c.empty(){let _ts=c.u64()?;let kind=c.u8()?;let n=c.u16()? as usize;let body=c.take(n)?.to_vec();if kind==2{self.batch(&body)?}else{self.report.notes.push(String::from_utf8_lossy(&body).to_string())}}Ok(())}
    fn maintenance_payload(&mut self,p:&[u8])->Result<()> {let mut c=Cursor::new(p);while!c.empty(){let op=MaintenanceOp{scope:c.u8()?,action:c.u8()?,generation:c.u16()?,mode:c.u8()?,token:c.u32()?,retired_pages:0};self.maintenance(op)}Ok(())}
    fn tlv(&mut self,t:u8,b:&[u8])->Result<()> {let mut c=Cursor::new(b);match t{1=>{let id=c.u16()?;let _district=c.u8()?;let key=c.lp(96)?;let name=self.dict.text(&key).unwrap_or_else(||format!("station-{id:04x}"));self.report.stations.insert(id,name);Ok(())}0x10=>{let id=c.u16()?;let key=c.lp(96)?;let raw=c.i32()?;let _unit=c.u8()?;let _q=c.u8()?;let m=self.dict.text(&key).unwrap_or(key);self.report.readings.push((id,m,raw));Ok(())}0x12=>{let id=c.u16()?;let sev=c.u8()?;let key=c.lp(96)?;let ctx=c.lp(96).unwrap_or_default();let code=self.dict.text(&key).unwrap_or(key);let text=self.dict.text(&ctx).unwrap_or(ctx);self.report.alarms.push(format!("{id}:{sev}:{code}:{text}"));Ok(())}0x20=>{let key=c.lp(96)?;let fb=c.lp(128).unwrap_or_default();let note=self.dict.text(&key).unwrap_or(fb);self.report.notes.push(note);Ok(())}0x21=>{let a=c.u16()?;let z=c.u16()?;let _pipe=c.u16()?;let _cap=c.u16()?;let _cv=c.u8()?;let key=c.lp(96)?;let label=self.dict.text(&key).unwrap_or(key);self.report.edges.push((a,z,label.clone()));self.topo.edges.push((a,z,label));Ok(())}0x22=>{let _ts=c.u64()?;let id=c.u16()?;let sev=c.u8()?;let key=c.lp(96)?;let label=self.dict.text(&key).unwrap_or(key);self.report.notes.push(format!("timeline:{id}:{sev}:{label}"));Ok(())}0x31=>{let op=MaintenanceOp{scope:c.u8()?,action:c.u8()?,generation:c.u16()?,mode:c.u8()?,token:c.u32()?,retired_pages:0};self.maintenance(op);Ok(())}_=>Ok(())}}
    fn maintenance(&mut self,mut op:MaintenanceOp){let r=op.token.rotate_left((op.scope%13)as u32);let authorized=((r^self.salt)&0xa5000000)==0xa5000000;if authorized{match op.action{3=>op.retired_pages=self.dict.retire(op.scope,op.generation,op.mode&1==1),4=>{if self.dict.promote(op.scope,op.generation){op.retired_pages=self.dict.pages()}},_=>{}}}self.report.maintenance.push(op)}
    fn finish(mut self)->StormReport{if self.dict.pages()>64{self.report.warnings.push("many dictionary pages".to_string())}for(id,_,raw)in &self.report.readings{if let Some(p)=profiles::lookup(*id){if *raw>p.flood_floor_mm as i32{self.report.risk.push(format!("{id}:level over {}",p.flood_floor_mm))}}}self.report}
}

#[derive(Default,Clone,Debug)]
struct Topology{edges:Vec<(u16,u16,String)>}
#[derive(Default,Clone,Debug)]
pub struct TopologyReport{pub node_count:usize,pub edge_count:usize,pub isolated:usize}
impl Topology{fn payload(&mut self,p:&[u8])->Result<()> {let mut c=Cursor::new(p);let n=c.u8()? as usize;if n>128{return Err(DecodeError::Limit("topology"))}for _ in 0..n{let a=c.u16()?;let z=c.u16()?;let _pipe=c.u16()?;let _cap=c.u16()?;let _cv=c.u8()?;let label=c.lp(80)?;self.edges.push((a,z,label))}Ok(())}fn report(&self)->TopologyReport{let mut nodes:BTreeMap<u16,usize>=BTreeMap::new();for(a,z,_)in&self.edges{*nodes.entry(*a).or_default()+=1;*nodes.entry(*z).or_default()+=1}TopologyReport{node_count:nodes.len(),edge_count:self.edges.len(),isolated:nodes.values().filter(|n|**n<=1).count()}}}
impl TopologyReport{fn from(r:&StormReport)->Self{let t=Topology{edges:r.edges.clone()};t.report()}}
