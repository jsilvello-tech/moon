use moon_monitor::monitoring::circular_buffer::CircularBuffer;

#[test]
fn test_circular_buffer_capacity_and_overflow() {
    let mut buf = CircularBuffer::new(3);
    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());

    buf.push(10.0);
    buf.push(20.0);
    assert_eq!(buf.len(), 2);
    assert_eq!(buf.to_vec(), vec![10.0, 20.0]);

    buf.push(30.0);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.to_vec(), vec![10.0, 20.0, 30.0]);

    // O 4º item deve expulsar o mais antigo (10.0)
    buf.push(40.0);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.to_vec(), vec![20.0, 30.0, 40.0]);
    assert_eq!(buf.latest(), Some(40.0));
}

#[test]
fn test_circular_buffer_resize() {
    let mut buf = CircularBuffer::new(5);
    for i in 1..=5 {
        buf.push(i as f32);
    }
    assert_eq!(buf.len(), 5);

    // Redimensiona para capacidade menor (3): deve manter os 3 mais recentes
    buf.resize(3);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.to_vec(), vec![3.0, 4.0, 5.0]);
}

#[test]
fn test_circular_buffer_clear() {
    let mut buf = CircularBuffer::new(10);
    buf.push(42.0);
    assert!(!buf.is_empty());
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.latest(), None);
}
