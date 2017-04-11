use super::internal::*;
use super::*;

use std::iter;

mod test;

/// `InScheduler` is a parallel iterator that switches to use the
/// given scheduler, rather than the default (rayon-core).
///
/// This struct is created by the [`in_scheduler()`] method on [`ParallelIterator`]
///
/// [`in_scheduler()`]: trait.ParallelIterator.html#method.in_scheduler
/// [`ParallelIterator`]: trait.ParallelIterator.html
pub struct InScheduler<I, S>
    where I: ParallelIterator<Scheduler = DefaultScheduler>,
          S: Scheduler,
{
    base: I,
    scheduler: S,
}

/// Create a new `InScheduler` iterator.
///
/// NB: a free fn because it is NOT part of the end-user API.
pub fn new<I, S>(base: I, scheduler: S) -> InScheduler<I, S>
    where I: ParallelIterator<Scheduler = DefaultScheduler>, S: Scheduler,
{
    InScheduler { base: base, scheduler: scheduler }
}

impl<I, S> ParallelIterator for InScheduler<I, S>
    where I: ParallelIterator<Scheduler = DefaultScheduler>,
          S: Scheduler,
{
    type Item = I::Item;
    type Scheduler = Selected<S>;

    fn drive_unindexed<C, S1>(self, consumer: C, _scheduler: S1) -> C::Result
        where C: UnindexedConsumer<Self::Item>, S: Scheduler,
    {
        self.base.drive_unindexed(consumer, self.scheduler)
    }

    fn opt_len(&mut self) -> Option<usize> {
        self.base.opt_len()
    }
}

impl<I, S> IndexedParallelIterator for InScheduler<I, S>
    where I: IndexedParallelIterator<Scheduler = DefaultScheduler>,
          S: Scheduler,
{
    fn drive<C, S1>(self, consumer: C, _scheduler: S1) -> C::Result
        where C: Consumer<Self::Item>, S: Scheduler,
    {
        self.base.drive(consumer, self.scheduler)
    }

    fn len(&mut self) -> usize {
        self.base.len()
    }

    fn with_producer<CB, S1>(self, callback: CB, _scheduler: S1) -> CB::Output
        where CB: ProducerCallback<Self::Item>, S: Scheduler,
    {
        self.base.with_producer(callback, self.scheduler)
    }
}

#[derive(Copy, Clone)]
pub struct Selected<S: Scheduler> {
    scheduler: S
}

impl<S: Scheduler> Scheduler for Selected<S> {
    fn execute_indexed<P, C>(self,
                             len: usize,
                             producer: P,
                             consumer: C)
                             -> C::Result
        where P: Producer,
              C: Consumer<P::Item>,
    {
        self.scheduler.execute_indexed(len, producer, consumer)
    }

    fn execute_unindexed<P, C>(self,
                               producer: P,
                               consumer: C)
                               -> C::Result
        where P: UnindexedProducer,
              C: UnindexedConsumer<P::Item>,
    {
        self.scheduler.execute_unindexed(producer, consumer)
    }
}


