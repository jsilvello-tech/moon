# 🌙 Moon Monitor

> **Keep your system in orbit.**

O **Moon Monitor** é um monitor de desempenho e telemetria de hardware nativo para **Windows 10 e 11**, desenvolvido em **Rust** com o moderno framework de interface gráfica **Slint**. 

Projetado para aliar alto desempenho, design futurista inspirado na exploração espacial e baixo consumo de recursos, o Moon Monitor coleta dados reais do computador sem exigir permissões de administrador nem depender de navegadores, Electron, WebView ou conexão à internet.

---

## 🚀 Funcionalidades

1. **🪐 Moon Dashboard (Visão Geral):**
   - Telemetria em tempo real agregada de CPU, Memória RAM, GPU e Armazenamento.
   - Indicador dinâmico de **Estado Lunar** (*Lunar Status*).
   - Identificação do computador, versão do Windows, arquitetura e tempo de atividade (*uptime*).
   - Mini-gráficos (*sparklines*) de histórico dos últimos 60 segundos.

2. **⚡ Lunar Core (Processador):**
   - Utilização total e decomposição detalhada por cada núcleo lógico.
   - Detecção de modelo, fabricante, contagem de núcleos físicos e threads.
   - Frequência de clock e temperatura quando suportadas pela plataforma.
   - Histórico contínuo com amostragem configurável.

3. **💎 Crystal Memory (Memória RAM & Paginação):**
   - Exibição de memória total, utilizada, disponível e livre em **GiB**.
   - Monitoramento do arquivo de paginação (*Swap / Pagefile*).
   - Ranking em tempo real dos maiores processos consumidores de memória.
   - Diferenciação precisa entre memória livre e memória disponível (reaproveitável por cache).

4. **🎮 Eclipse Graphics (Adaptadores Gráficos / GPU):**
   - Identificação nativa via **Windows DXGI** (DirectX Graphics Infrastructure) para GPUs NVIDIA, AMD, Intel e Qualcomm.
   - Métricas de VRAM dedicada e memória de sistema compartilhada.
   - Arquitetura modular resiliente com tratamento elegante de métricas proprietárias ("Não disponível").

5. **💾 Moon Storage (Armazenamento Não-Invasivo):**
   - Detecção de volumes, letras de unidade, rótulos e tipo de sistema de arquivos (NTFS, FAT32, exFAT).
   - Capacidade total, espaço utilizado e espaço livre.
   - **Zero desgaste:** não executa testes de estresse ou escritas artificiais em SSDs/HDDs.

6. **🚀 Orbit Processes (Processos do Sistema):**
   - Listagem completa de processos em execução com PID, uso de CPU e RAM.
   - Filtro de pesquisa instantâneo por nome do executável.
   - Ordenação dinâmica por maior consumo de CPU ou de Memória.
   - Inspetor de processo selecionado.
   - **Segurança:** Operação estritamente em modo somente leitura (sem permissão de encerramento de processos).

7. **⚙️ Lunar Settings (Configurações e Auditoria):**
   - Seleção de intervalo de amostragem: **1s**, **2s (padrão)** ou **5s**.
   - Alternância entre tema escuro (*Midnight Lunar*) e tema claro (*Solar Day*).
   - Ativação ou desativação de microanimações.
   - Pausa e retomada instantânea da coleta de telemetria.
   - **Auto-monitoramento:** Diagnóstico do consumo de CPU e RAM do próprio Moon Monitor.
   - Persistência automática em `%APPDATA%\MoonMonitor\settings.json`.

---

## 🎨 Identidade Visual e Paleta Lunar

| Elemento | Código Hex | Aplicação |
|---|---|---|
| **Midnight Blue** | `#0B1020` | Fundo principal da aplicação |
| **Deep Space** | `#141B30` | Superfície de cartões e barra lateral |
| **Moonlight Silver** | `#DDE5F3` | Tipografia primária e realces de cabeçalho |
| **Lunar Violet** | `#A78BFA` | Acentos da CPU e destaque de marca |
| **Crystal Cyan** | `#67E8F9` | Acentos de RAM e linhas de telemetria |
| **Orbit Green** | `#4ADE80` | Status de operação normal e discos |
| **Solar Warning** | `#FBBF24` | Cargas elevadas (70% - 89%) e GPU |
| **Eclipse Red** | `#FB7185` | Carga crítica (≥ 90%) e alertas |

### Indicador Lunar de Carga (Lunar Status)
- **Lua Crescente:** Carga de CPU inferior a 30%.
- **Lua Gibosa:** Carga de CPU entre 30% e 69%.
- **Lua Cheia:** Carga de CPU entre 70% e 89%.
- **Eclipse:** Carga de CPU igual ou superior a 90%.

---

## 🛠️ Requisitos de Compilação

Para compilar o Moon Monitor a partir do código-fonte no Windows:

1. **Rust:** Toolchain estável instalada via [rustup](https://rustup.rs/) (`x86_64-pc-windows-msvc`).
2. **Microsoft C++ Build Tools:** Componente do Visual Studio (C++ x64/x86 build tools e Windows 10/11 SDK).
3. **Cargo:** Gerenciador padrão de pacotes do Rust.

---

## 💻 Instruções de Execução e Compilação

Abra o terminal PowerShell na pasta do projeto:

### 1. Verificação de código
```powershell
cargo check
```

### 2. Verificação de formatação
```powershell
cargo fmt --check
```

### 3. Análise estática com Clippy
```powershell
cargo clippy --all-targets -- -D warnings
```

### 4. Execução dos testes automatizados
```powershell
cargo test
```

### 5. Execução em modo de desenvolvimento
```powershell
cargo run
```

### 6. Compilação otimizada para lançamento (Release)
```powershell
cargo build --release
```

O executável portátil final será gerado em:
```text
target/release/moon-monitor.exe
```

---

## 📦 Distribuição Portátil e Instalador

### Distribuição Portátil
O binário `moon-monitor.exe` gerado em `target/release/` é autossuficiente e portátil. Ele pode ser copiado para qualquer pasta ou pendrive e executado em qualquer máquina com Windows 10 ou 11 x64 sem necessidade de instalação.

### Gerando o Instalador Windows (.exe)
Um script de empacotamento completo para o **Inno Setup** está disponível em `installer/moon_monitor.iss`.

Para compilar o instalador:
1. Instale o [Inno Setup 6](https://jrsoftware.org/isdl.php).
2. Execute o comando:
```powershell
iscc installer/moon_monitor.iss
```
O instalador será criado na pasta `installer/Output/MoonMonitor-v0.1.0-Setup-x64.exe`. Ele é configurado com `PrivilegesRequired=lowest` para instalação por usuário, sem necessidade de elevação de privilégios UAC.

---

## 🔬 Metodologia de Coleta vs. Gerenciador de Tarefas do Windows

Podem ocorrer pequenas variações entre as porcentagens exibidas pelo Moon Monitor e pelo Gerenciador de Tarefas do Windows (*Task Manager*):
- **Janela de Amostragem:** O Gerenciador de Tarefas atualiza por padrão a cada 1 segundo com cálculos proporcionais de frequência turbo do Windows Kernel. O Moon Monitor atualiza a cada 2 segundos (ou 1s/5s conforme configuração), calculando o delta de ciclos de tempo não-ocioso entre duas amostragens discretas.
- **Memória RAM:** O Moon Monitor diferencia rigorosamente **Memória Utilizada**, **Memória Disponível** (que inclui cache de páginas que o Windows pode liberar a qualquer momento) e **Memória Total**, evitando a confusão comum entre memória livre e memória utilizável.

---

## ⚠️ Limitações Conhecidas (v0.1.0)

- **Sensores de Temperatura de CPU/Placa-Mãe:** O Windows não fornece uma API padronizada e aberta para leitura de sensores térmicos sem acesso a drivers em modo kernel ou ACPI proprietário. Caso a placa-mãe não exponha sensores através da interface padrão, o campo exibirá com segurança *"Não disponível"*.
- **Utilização e Clock detalhado de GPU:** O monitoramento via DXGI fornece o nome exato, fabricante e quantia de memória de vídeo dedicada e compartilhada de qualquer GPU no sistema. Métricas como clock dinâmico e temperatura da GPU dependem de SDKs específicos dos fabricantes (NVIDIA NVML, AMD ADL), que não são incluídos nativamente nesta primeira versão experimental.

---

## 📄 Licença

Este projeto é disponibilizado sob a licença **MIT**. Consulte o arquivo [LICENSE](LICENSE) para mais detalhes.
