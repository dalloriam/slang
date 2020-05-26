use criterion::{black_box, criterion_group, criterion_main, Criterion};

use assembler::Assembler;
use vm::VM;

fn execute_test(source: Vec<u8>, max_epochs: usize) -> VM {
    let mut vm = VM::new();
    vm.load_bytecode(source).unwrap();

    let mut current_epochs = 0;
    let mut keepalive = true;
    while keepalive {
        keepalive = vm.run_once();
        current_epochs += 1;
        if current_epochs >= max_epochs {
            panic!("Test took too long");
        }
    }

    vm
}

fn bench_tight_loop(c: &mut Criterion) {
    // VM Setup
    const SOURCE: &str = include_str!("./data/tight_loop.asm");
    let asm = Assembler::new().assemble(SOURCE).unwrap();
    c.bench_function("tight loop", |b| {
        b.iter(|| execute_test(asm.clone(), black_box(200000)))
    });
}

fn bench_stack_push_pop(c: &mut Criterion) {
    // VM Setup
    const SOURCE: &str = include_str!("./data/stack.asm");
    let asm = Assembler::new().assemble(SOURCE).unwrap();
    c.bench_function("stack push/pop", |b| {
        b.iter(|| execute_test(asm.clone(), black_box(200000)))
    });
}

fn bench_heap_alloc(c: &mut Criterion) {
    // VM Setup
    const SOURCE: &str = include_str!("./data/heap_alloc.asm");
    let asm = Assembler::new().assemble(SOURCE).unwrap();
    c.bench_function("heap alloc", |b| {
        b.iter(|| execute_test(asm.clone(), black_box(200000)))
    });
}

criterion_group!(
    benches,
    bench_tight_loop,
    bench_stack_push_pop,
    bench_heap_alloc
);
criterion_main!(benches);
