
// --------==========[ Base Traits ]==========--------

use std::marker::PhantomData;

/// Right folded HList
pub trait RHList: Sized {
    const LENGTH: usize;

    fn prepend<H>(self, h: H) -> (H, Self) {
        (h, self)
    }
}

impl RHList for () {
    const LENGTH: usize = 0;
}

impl<H, T: RHList> RHList for (H, T) {
    const LENGTH: usize = T::LENGTH + 1;
}

/// Left folded HList
pub trait LHList: Sized {
    const LENGTH: usize;
    
    fn append<T>(self, t: T) -> (Self, T) {
        (self, t)
    }
}

impl LHList for () {
    const LENGTH: usize = 0;
}

impl<T: LHList, H> LHList for (T, H) {
    const LENGTH: usize = T::LENGTH + 1;
}


// --------==========[ Prepend / Append ]==========--------

/// Preprended LHList with element.
pub trait LHListPrepend<E>: LHList {
    type Preprended: LHList;

    fn prepend(self, elem: E) -> Self::Preprended;
}

/// Base case
impl<E> LHListPrepend<E> for () {
    type Preprended = ((), E);
    
    fn prepend(self, elem: E) -> Self::Preprended {
        (self, elem)
    }
}

/// Inductive step
impl<E, H: LHListPrepend<E>, T> LHListPrepend<E> for (H, T) {
    type Preprended = (<H as LHListPrepend<E>>::Preprended, T);
    
    fn prepend(self, elem: E) -> Self::Preprended {
        let (head, tail) = self;
        (head.prepend(elem), tail)
    }
}

/// Append for RHList.
pub trait RHListAppend<E>: RHList {
    type Appended: RHList;

    fn append(self, elem: E) -> Self::Appended;
}

/// Base case
impl<E> RHListAppend<E> for () {
    type Appended = (E, ());

    fn append(self, elem: E) -> Self::Appended {
        (elem, ())
    }
}

/// Inductive step
impl<E, H, T: RHListAppend<E>> RHListAppend<E> for (H, T) {
    type Appended = (H, <T as RHListAppend<E>>::Appended);

    fn append(self, elem: E) -> Self::Appended {
        let (head, tail) = self;
        (head, tail.append(elem))
    }
}


// --------==========[ First / Last ]==========--------

/// Getter for last element
pub trait RHListLast: RHList {
    type Last;

    fn last(&self) -> &Self::Last;
}

/// Base case
impl<E> RHListLast for (E, ()) {
    type Last = E;

    fn last(&self) -> &Self::Last {
        &self.0
    }
}

/// Inductive step
impl<H, E, T> RHListLast for (H, (E, T))
where
    (E, T): RHListLast,
{
    type Last = <(E, T) as RHListLast>::Last;

    fn last(&self) -> &Self::Last {
        self.1.last()
    }
}

/// Getter for first element
pub trait LHListFirst: LHList {
    type First;

    fn first(&self) -> &Self::First;
}

impl<E> LHListFirst for ((), E) {
    type First = E;

    fn first(&self) -> &Self::First {
        &self.1
    }
}

impl<H, E, T> LHListFirst for ((H, E), T)
where
    (H, E): LHListFirst
{
    type First = <(H, E) as LHListFirst>::First;

    fn first(&self) -> &Self::First {
        self.0.first()
    }
}


// --------==========[ Inversion LHList <-> RHList ]==========--------

/// LHList conversion to RHList
pub trait LHListInvert: LHList {
    type Inverted: RHList;

    fn invert(self) -> Self::Inverted;
}

/// Base case
impl LHListInvert for () {
    type Inverted = ();

    fn invert(self) -> Self::Inverted {
        self
    }
}

/// Inductive step
impl<H: LHListInvert, E> LHListInvert for (H, E)
where
    H::Inverted: RHListAppend<E>
{
    type Inverted = <H::Inverted as RHListAppend<E>>::Appended;

    fn invert(self) -> Self::Inverted {
        let (head, elem) = self;
        head.invert().append(elem)
    }
}

/// RHList conversion to LHList.
pub trait RHListInvert: RHList {
    type Inverted: LHList;

    fn invert(self) -> Self::Inverted;
}

/// Base case
impl RHListInvert for () {
    type Inverted = ();

    fn invert(self) -> Self::Inverted {
        ()
    }
}

/// Inductive step
impl<E, T: RHListInvert> RHListInvert for (E, T)
where
    T::Inverted: LHListPrepend<E>
{
    type Inverted = <T::Inverted as LHListPrepend<E>>::Preprended;

    fn invert(self) -> Self::Inverted {
        let (elem, tail) = self;
        tail.invert().prepend(elem)
    }
}


// --------==========[ HList Reversion ]==========--------

/// Reverse LHList
pub trait LHListReverse: LHList {
    type Reversed: LHList;

    fn reverse(self) -> Self::Reversed;
}

/// Base case
impl LHListReverse for () {
    type Reversed = ();

    fn reverse(self) -> Self::Reversed {
        self
    }
}

/// Inductive step
impl<H: LHListReverse, E> LHListReverse for (H, E)
where
    H::Reversed: LHListPrepend<E>
{
    type Reversed = <H::Reversed as LHListPrepend<E>>::Preprended;
    
    fn reverse(self) -> Self::Reversed {
        let (head, elem) = self;
        head.reverse().prepend(elem)
    }
}

/// Reverse RHList
pub trait RHListReverse {
    type Reversed: RHList;

    fn reverse(self) -> Self::Reversed;
}

/// Base case
impl RHListReverse for () {
    type Reversed = ();
    
    fn reverse(self) -> Self::Reversed {
        self
    }
}

/// Inductive step
impl<E, T: RHList + RHListReverse> RHListReverse for (E, T)
where
    T::Reversed: RHListAppend<E>,
{
    type Reversed = <T::Reversed as RHListAppend<E>>::Appended;
    
    fn reverse(self) -> Self::Reversed {
        let (elem, tail) = self;
        tail.reverse().append(elem)
    }
}


// --------==========[ Type Level Counters ]==========--------

pub trait Index {
    const INDEX: usize;
}

pub struct Zero;
impl Index for Zero {
    const INDEX: usize = 0;
}

pub struct Successor<I: Index>(PhantomData<I>);
impl<I: Index> Index for Successor<I> {
    const INDEX: usize = I::INDEX + 1;
}


// --------==========[ HList Selectors ]==========--------

pub trait LHListSelector<Needle, I>: LHList
where
    I: Index
{
    fn get(&self) -> &Needle;

    fn get_mut(&mut self) ->&mut Needle;
}

impl<H: LHList, Needle> LHListSelector<Needle, Zero> for (H, Needle) {
    fn get(&self) -> &Needle {
        &self.1
    }

    fn get_mut(&mut self) -> &mut Needle {
        &mut self.1
    }
}

impl<H, T, Needle, I> LHListSelector<Needle, Successor<I>> for (H, T)
where
    H: LHListSelector<Needle, I>,
    I: Index,
{
    fn get(&self) -> &Needle {
        self.0.get()
    }

    fn get_mut(&mut self) -> &mut Needle {
        self.0.get_mut()
    }
}

pub trait RHListSelector<Needle, I>: RHList
where
    I: Index
{
    fn get(&self) -> &Needle;

    fn get_mut(&mut self) -> &mut Needle;
}

impl<Needle, T: RHList> RHListSelector<Needle, Zero> for (Needle, T) {
    fn get(&self) -> &Needle {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Needle {
        &mut self.0
    }
}

impl<H, T, Needle, I> RHListSelector<Needle, Successor<I>> for (H, T)
where
    T: RHListSelector<Needle, I>,
    I: Index
{
    fn get(&self) -> &Needle {
        self.1.get()
    }

    fn get_mut(&mut self) -> &mut Needle {
        self.1.get_mut()
    }
}


// --------==========[ Indexed HLists ]==========--------

pub struct Indexed<const INDEX: usize, T>(T);

impl<const INDEX: usize, T> Indexed<INDEX, T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

pub trait LHListIndexed: LHList { }

impl LHListIndexed for () { }
impl<const INDEX: usize, H: LHListIndexed, E> LHListIndexed for (H, Indexed<INDEX, E>) { }

pub trait RHListIndexed: RHList { }

impl RHListIndexed for () { }
impl<const INDEX: usize, E, T: RHListIndexed> RHListIndexed for (Indexed<INDEX, E>, T) { }


// --------==========[ Index HList Append ]==========--------

/// Appending new Indexed Element to LHList
pub trait LHListAppendIndexed<const INDEX: usize, E>: LHListIndexed {
    type AppendedIndexed: LHListIndexed;

    fn append_indexed(self, value: E) -> Self::AppendedIndexed;
}

/// Base case
impl<E> LHListAppendIndexed<0, E> for () {
    type AppendedIndexed = ((), Indexed<0, E>);

    fn append_indexed(self, value: E) -> Self::AppendedIndexed {
        ((), Indexed::new(value))
    }
}

/// Inductive step
impl<const INDEX: usize, H, T, E> LHListAppendIndexed<INDEX, E> for (H, T)
where
    (H, T): LHListIndexed
{
    type AppendedIndexed = ((H, T), Indexed<INDEX, E>);

    fn append_indexed(self, value: E) -> Self::AppendedIndexed {
        (self, Indexed::new(value))
    }
}

pub trait RHListAppendIndexed<const INDEX: usize, E>: RHListIndexed {
    type AppendedIndexed: RHListIndexed;

    fn append_indexed(self, value: E) -> Self::AppendedIndexed;
}

/// Base case
impl<E> RHListAppendIndexed<0, E> for () {
    type AppendedIndexed = (Indexed<0, E>, ());

    fn append_indexed(self, value: E) -> Self::AppendedIndexed {
        (Indexed::new(value), ())
    }
}

/// Inductive step
impl<const INDEX: usize, H, T, E> RHListAppendIndexed<INDEX, E> for (H, T)
where
    (H, T): RHListIndexed
{
    type AppendedIndexed = (Indexed<INDEX, E>, (H, T));

    fn append_indexed(self, value: E) -> Self::AppendedIndexed {
        (Indexed::new(value), self)
    }
}

// TODO: custom display for HLists? On vscode level? for example `((((), i32), u32), f32)`` would become `[i32, u32, f32]`?

fn _test() {
    let lhlist = ((((), 1i32), 2u32), 3f32);
    let _prepended_lhlist = lhlist.clone().prepend("siema");
    let _reversed_lhlist = lhlist.clone().reverse();
    let _inverted_lhlist: (i32, (u32, (f32, ()))) = lhlist.clone().invert();
    
    let indexed = ()
        .append_indexed(1f32)
        .append_indexed("sigma");


    let rhlist = (1i32, (2u32, (3f32, ())));
    let _prepended_rhlist = rhlist.clone().prepend("sigma");
    let _reversed_rhlist = rhlist.clone().reverse();
    let _inverted_rhlist = rhlist.clone().invert();
}
