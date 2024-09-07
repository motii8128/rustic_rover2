use std::cell::RefCell;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use std::sync::mpsc::{channel, Sender, Receiver};

type Publisher<T> = Sender<T>;
type Subscriber<T> = Receiver<T>;
type AsyncPublisher<T> = UnboundedSender<T>;
type AsyncSubscriber<T> = UnboundedReceiver<T>;

pub struct Node<T>
{
    pub publisher:Publisher<T>,
    pub subscriber:Subscriber<T>
}

impl<T> Node<T> {
    pub fn new()->Node<T>
    {
        let (t, r) = channel::<T>();

        Node { publisher: t, subscriber: r }
    }
}

pub struct AsyncNode<T>
{
    pub publisher: RefCell<Option<AsyncPublisher<T>>>,
    pub subscriber: RefCell<Option<AsyncSubscriber<T>>>
}

impl<T> AsyncNode<T> {
    pub fn new()->AsyncNode<T>
    {
        let (p, s) = unbounded_channel::<T>();

        AsyncNode { publisher: RefCell::new(Some(p)), subscriber: RefCell::new(Some(s)) }
    }

    pub fn get_publisher(&self)->AsyncPublisher<T>
    {
        self.publisher.clone().take().unwrap()
    }
    pub fn get_subscriber(&self)->Option<AsyncSubscriber<T>>
    {
        self.subscriber.take()
    }
}