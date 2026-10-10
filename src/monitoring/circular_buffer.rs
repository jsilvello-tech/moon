//! Buffer circular de tamanho fixo para manter o histórico de telemetria dos últimos 60 segundos.

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct CircularBuffer {
    capacity: usize,
    buffer: VecDeque<f32>,
}

impl CircularBuffer {
    /// Cria um novo buffer circular com capacidade máxima especificada.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            capacity: cap,
            buffer: VecDeque::with_capacity(cap),
        }
    }

    /// Adiciona um novo ponto de telemetria. Se a capacidade for atingida, remove o ponto mais antigo.
    pub fn push(&mut self, value: f32) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
    }

    /// Redefine a capacidade máxima do buffer preservando os dados mais recentes.
    pub fn resize(&mut self, new_capacity: usize) {
        let new_cap = new_capacity.max(1);
        self.capacity = new_cap;
        while self.buffer.len() > self.capacity {
            self.buffer.pop_front();
        }
    }

    /// Retorna uma cópia dos valores em ordem cronológica (do mais antigo ao mais recente).
    pub fn to_vec(&self) -> Vec<f32> {
        self.buffer.iter().copied().collect()
    }

    /// Retorna o número de amostras atualmente contidas no buffer.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Retorna se o buffer está vazio.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Limpa todas as amostras acumuladas.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Retorna o último valor inserido, se houver.
    pub fn latest(&self) -> Option<f32> {
        self.buffer.back().copied()
    }
}
