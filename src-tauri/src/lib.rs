use ::tokio::runtime::Builder;
use libp2p::{swarm::{dummy::Behaviour as DummyBehaviour, Swarm}, Multiaddr, SwarmBuilder, futures::StreamExt};

pub fn create_swarm() -> Swarm<DummyBehaviour> {
    SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            Default::default(),
            libp2p::tls::Config::new, 
            libp2p::yamux::Config::default
        )
        .unwrap()
        .with_behaviour(|_| DummyBehaviour)
        .unwrap()
        .build()
}

#[test]
fn test_ping() {
    let mut swarm0 = create_swarm();
    let mut  swarm1 = create_swarm();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.spawn(async move {
        swarm0
            .listen_on("/ip4/127.0.0.1/tcp/8888".parse().unwrap())
            .unwrap();

        while let Some(event) = swarm0.next().await {
            println!("swarm0: {:?}", event);
        }
    });

    runtime.block_on(async move {
        // Wait for the listener to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        swarm1.dial("/ip4/127.0.0.1/tcp/8888".parse::<Multiaddr>().unwrap()).unwrap();
        while let Some(event) = swarm1.next().await {
            println!("swarm1: {:?}", event);
        }
    });
}