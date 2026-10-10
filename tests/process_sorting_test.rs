use moon_monitor::models::process::{ProcessDetail, ProcessSortOrder};

#[test]
fn test_process_sort_by_cpu() {
    let mut list = vec![
        ProcessDetail {
            pid: 100,
            name: "idle.exe".to_string(),
            cpu_usage_percent: 1.2,
            memory_bytes: 50_000_000,
            disk_read_bytes: 0,
            disk_written_bytes: 0,
        },
        ProcessDetail {
            pid: 200,
            name: "render.exe".to_string(),
            cpu_usage_percent: 78.5,
            memory_bytes: 10_000_000,
            disk_read_bytes: 0,
            disk_written_bytes: 0,
        },
        ProcessDetail {
            pid: 300,
            name: "compiler.exe".to_string(),
            cpu_usage_percent: 42.0,
            memory_bytes: 80_000_000,
            disk_read_bytes: 0,
            disk_written_bytes: 0,
        },
    ];

    list.sort_by(|a, b| {
        b.cpu_usage_percent
            .partial_cmp(&a.cpu_usage_percent)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    assert_eq!(list[0].pid, 200); // 78.5%
    assert_eq!(list[1].pid, 300); // 42.0%
    assert_eq!(list[2].pid, 100); // 1.2%
}

#[test]
fn test_process_sort_by_memory() {
    let mut list = vec![
        ProcessDetail {
            pid: 100,
            name: "browser.exe".to_string(),
            cpu_usage_percent: 5.0,
            memory_bytes: 500_000_000,
            disk_read_bytes: 0,
            disk_written_bytes: 0,
        },
        ProcessDetail {
            pid: 200,
            name: "editor.exe".to_string(),
            cpu_usage_percent: 2.0,
            memory_bytes: 150_000_000,
            disk_read_bytes: 0,
            disk_written_bytes: 0,
        },
        ProcessDetail {
            pid: 300,
            name: "vm.exe".to_string(),
            cpu_usage_percent: 12.0,
            memory_bytes: 2_000_000_000,
            disk_read_bytes: 0,
            disk_written_bytes: 0,
        },
    ];

    list.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));

    assert_eq!(list[0].pid, 300); // 2 GB
    assert_eq!(list[1].pid, 100); // 500 MB
    assert_eq!(list[2].pid, 200); // 150 MB
}
