
#[derive(Debug, Default, Clone)]
pub struct Car {
    owner: String,
    lasttrain: usize,
    prevtrain: usize,
    location: usize,
    destination: usize,
    marks: String,
    number: String,
    divisions: String,
    length: u8,
    plate: u8,
    weightclass: u8,
    ltwt: u8,
    ldlmt: u8,
    trips: u32,
    moves: u32,
    assignments: u32,
    loadedP: bool,
    mirrorP: bool,
    fixedP: bool,
    doneP: bool,
    peek: bool,
    tmpStatus: bool,
    cartype: char,
}

use std::fmt;

impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.marks, self.number)
    }
}

impl Car {
    pub fn new(t: char, m: String, n: String, d: String, l: u8, p: u8, 
               wc: u8, lw: u8, ldw: u8, lp: bool, mp: bool, fp: bool,
               own: String, dp: bool,lt: usize,mvs: u32, loc: usize,
               dest: usize, trps: u32, asgns: u32) -> Self {
        Self {owner: own, lasttrain: lt, prevtrain: lt, location: loc,
              destination: dest, marks: m, number: n, divisions: d,
              length: l, plate: p, weightclass: wc, ltwt: lw,
              ldlmt: ldw, trips: trps, moves: mvs, assignments: asgns,
              loadedP: lp, mirrorP: mp, fixedP: fp, doneP: dp, peek: false,
              tmpStatus: false, cartype: t}
    }
    pub fn Type(&self) -> char {self.cartype}
    pub fn SetType(&mut self,t: char) {self.cartype = t;}
    pub fn Marks(&self) -> String {self.marks.clone()}
    pub fn SetMarks(&mut self,m: String) {self.marks = m;}
    pub fn Number(&self) -> String {self.number.clone()}
    pub fn SetNumber(&mut self,n: String) {self.number = n;}
    pub fn Divisions(&self) -> String {self.divisions.clone()}
    pub fn SetDivisions(&mut self,d: String) {self.divisions = d;}
    pub fn Length(&self) -> u8 {self.length}
    pub fn SetLength(&mut self,l: u8) {self.length = l;}
    pub fn Plate(&self) -> u8 {self.plate}
    pub fn SetPlate(&mut self, p: u8) {self.plate = p;}
    pub fn WeightClass(&self) -> u8 {self.weightclass}
    pub fn SetWeightClass(&mut self, wc: u8) {self.weightclass = wc;}
    pub fn LtWt(&self) -> u8 {self.ltwt}
    pub fn SetLtWt(&mut self, lw: u8) {self.ltwt = lw;}
    pub fn LdLmt(&self) -> u8 {self.ldlmt}
    pub fn SetLdLmt(&mut self, ldw: u8) {self.ldlmt = ldw;}
    pub fn LoadedP(&self) -> bool {self.loadedP}
    pub fn EmptyP(&self) -> bool {!self.loadedP}
    pub fn Load(&mut self) {self.loadedP = true;}
    pub fn UnLoad(&mut self) {self.loadedP = false;}
    pub fn OkToMirrorP(&self) -> bool {self.mirrorP}
    pub fn SetOkToMirrorP(&mut self, m: bool) {self.mirrorP = m;}
    pub fn FixedRouteP(&self) -> bool {self.fixedP}
    pub fn SetFixedRouteP(&mut self, f: bool) {self.fixedP = f;}
    pub fn Owner(&self) -> String {self.owner.clone()}
    pub fn SetCarOwner(&mut self, o: String) {self.owner = o;}
    pub fn IsDoneP(&self) -> bool {self.doneP}
    pub fn SetDone(&mut self) {self.doneP = true;}
    pub fn SetNotDone(&mut self) {self.doneP = false;}
    pub fn LastTrain(&self) -> usize {self.lasttrain}
    pub fn SetLastTrain(&mut self, lt: usize) {self.lasttrain = lt;}
    pub fn PrevTrain(&self) -> usize {self.prevtrain}
    pub fn SetPrevTrain(&mut self, lt: usize) {self.prevtrain = lt;}
    pub fn MovementsThisSession(&self) -> u32 {self.moves}
    pub fn ClearMovementsThisSession(&mut self) {self.moves = 0;}
    pub fn IncrmentMovementsThisSession(&mut self) {self.moves = self.moves + 1;}
    pub fn Location(&self) -> usize {self.location}
    pub fn SetLocation(&mut self, newloc: usize) {self.location = newloc;}
    pub fn Destination(&self) -> usize {self.destination}
    pub fn SetDestination(&mut self, newdest: usize) {self.destination = newdest;}
    pub fn Trips(&self) -> u32 {self.trips}
    pub fn ClearTrips(&mut self) {self.trips = 0;}
    pub fn IncrementTrips(&mut self) {self.trips = self.trips + 1;}
    pub fn Assignments(&self) -> u32 {self.assignments}
    pub fn SetAssignments(&mut self, a: u32) {self.assignments = a;}
    pub fn ClearAssignments(&mut self) {self.assignments = 0;}
    pub fn IncrementAssignments(&mut self) {self.assignments = self.assignments + 1;}
    pub fn Peek(&self) -> bool {self.peek}
    pub fn SetPeek(&mut self, p: bool) {self.peek = p;}
}               
