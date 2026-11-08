# ALSEYE BEM - Cross-Platform Cooler & TFT Controller (Rust)

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform: Linux & Windows](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-green.svg)]()
[![Hardware: ALSEYE BEM](https://img.shields.io/badge/hardware-ALSEYE%20BEM%20I3.03-blueviolet.svg)]()

Implementação multiplataforma de alta performance desenvolvida em **Rust** para monitoramento de hardware e controle da tela TFT de watercoolers **ALSEYE BEM**, criada a partir da engenharia reversa e descompilação do software original `ALSEYE BEM.exe` (11.244 rotinas analisadas via REA e Ghidra).

Substitui o aplicativo original do Windows por uma versão leve, moderna, sem telemetria, com consumo insignificante de CPU e compatibilidade nativa com **Linux** e **Windows**.

---

## 📸 Funcionalidades

1. **Hardware Monitor**:
   - Mostradores circulares em tempo real para temperatura de CPU e GPU.
   - Barras dinâmicas de utilização (% de CPU e GPU), frequência de clock (MHz), rotação das ventoinhas (RPM) e consumo de energia (Watts).
2. **Hardware Information**:
   - Inventário detalhado: CPU, Placa-mãe, Memória RAM, Armazenamento, Placa de Vídeo, Áudio, Rede e Versão do Kernel/OS.
3. **TFT Screen Controller**:
   - Habilitação/desabilitação individual de métricas no display TFT.
   - Chave mestre On/Off da tela do cooler.
   - Controle de taxa de alternância (Speed: 1s, 3s, 5s).
   - Rotação do display (0°, 90°, 180°, 270°).
   - Pré-visualização gráfica simulada do display TFT no desktop.
4. **Comunicação USB HID Nativa**:
   - Reconhecimento automático do dispositivo (`VID: 0x5131`, `PID: 0x2007`).
   - Protocolo Standard (`0x40`) e protocolo de frame TFT (`0xC5` com checksum de verificação).

---

## 🐧 Guia de Implementação no Linux

O aplicativo foi projetado para rodar nativamente no Linux, acessando os sensores via kernel `sysfs` / `lm-sensors` e comunicando-se com o dispositivo USB HID através de `hidapi` sem necessidade de rodar como root.

### Método 1: Instalação Automática (Recomendado)

Clone o repositório e execute o script de instalação incluído:

```bash
git clone https://github.com/SEU-USUARIO/alseye-cross.git
cd alseye-cross
chmod +x linux/install.sh
./linux/install.sh
```

O script detecta automaticamente sua distribuição (Ubuntu, Debian, Fedora, Arch Linux), instala as bibliotecas necessárias, configura as regras `udev`, compila em modo Release e instala o binário em `/usr/local/bin/alseye-cross`.

---

### Método 2: Instalação Passo a Passo (Manual)

#### 1. Dependências do Sistema

- **Ubuntu / Debian / Linux Mint / Pop!_OS**:
  ```bash
  sudo apt update
  sudo apt install -y build-essential pkg-config libudev-dev libhidapi-dev lm-sensors
  ```

- **Fedora / RHEL**:
  ```bash
  sudo dnf install -y gcc gcc-c++ make pkgconf-pkg-config systemd-devel hidapi-devel lm_sensors
  ```

- **Arch Linux / Manjaro**:
  ```bash
  sudo pacman -S --needed base-devel pkgconf systemd hidapi lm_sensors
  ```

#### 2. Permissões de Acesso USB (Regras UDEV)

Por padrão, dispositivos USB brutos exigem privilégios de `root`. Para permitir que o seu usuário controle o watercooler sem usar `sudo`:

1. Copie o arquivo de regras:
   ```bash
   sudo cp linux/99-alseye.rules /etc/udev/rules.d/
   ```
   *(Conteúdo da regra: `SUBSYSTEM=="hidraw", ATTRS{idVendor}=="5131", ATTRS{idProduct}=="2007", MODE="0666", GROUP="plugdev"`)*

2. Recarregue as regras do subsistema UDEV:
   ```bash
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```

3. Adicione o seu usuário ao grupo `plugdev` ou `dialout`:
   ```bash
   sudo usermod -aG plugdev $USER
   sudo usermod -aG dialout $USER
   ```
   *(Faça logout e login novamente para aplicar a mudança de grupo).*

#### 3. Compilação e Execução

Certifique-se de ter o [Rust instalado](https://rustup.rs/):
```bash
cargo build --release
./target/release/alseye-cross
```

---

### ⚙️ Inicialização Automática no Linux (Systemd User Service)

Para que o monitor do cooler inicie automaticamente quando você fizer login no ambiente gráfico:

1. Copie o arquivo de serviço para sua pasta de usuário:
   ```bash
   mkdir -p ~/.config/systemd/user
   cp linux/alseye-cross.service ~/.config/systemd/user/
   ```

2. Certifique-se de que o executável está no caminho do sistema:
   ```bash
   sudo cp target/release/alseye-cross /usr/local/bin/
   ```

3. Habilite e inicie o serviço:
   ```bash
   systemctl --user daemon-reload
   systemctl --user enable --now alseye-cross.service
   ```

Para verificar o status:
```bash
systemctl --user status alseye-cross.service
```

---

## 🪟 Guia de Uso no Windows

1. Certifique-se de ter o [Rust](https://rustup.rs/) instalado (caso queira compilar a partir do código fonte).
2. Execute no PowerShell ou Prompt de Comando:
   ```powershell
   cargo run --release
   ```
3. Alternativamente, basta dar um duplo clique no arquivo **`executar.bat`**.

---

## 🔬 Especificação Técnica e Protocolo USB

A comunicação USB foi mapeada a partir da rotina `0x40a340` e `0x40bd30` descompiladas do binário original:

- **Endereço USB**: `VID: 0x5131`, `PID: 0x2007` (Interface HID Interrupt OUT).
- **Modo Standard (0x40)**:
  - Buffer de 65 bytes: `[0x00, 0x40, cpu_temp, ...]`.
- **Modo Tela TFT (0xC5)**:
  - Pacote de 18 bytes:
    - Byte 0: `0xC5` (Identificador de frame TFT)
    - Byte 1: Orientação da tela (`0x02` = 0°, `0x12` = 90°, `0x22` = 180°, `0x32` = 270°)
    - Bytes 2..15: Métricas ativas selecionadas (temperaturas, RPM, relógio)
    - Byte 16: Checksum calculado sobre os bytes anteriores
    - Byte 17: Delimitador de fim de pacote

---

## 🛠️ Resolução de Problemas no Linux (Troubleshooting)

* **Erro `Access denied (insufficient permissions)`**:
  - Certifique-se de que a regra em `/etc/udev/rules.d/99-alseye.rules` foi criada e que seu usuário faz parte do grupo `plugdev`. Desconecte e reconecte o cabo USB do cooler.
* **Temperatura da GPU não aparece**:
  - Em GPUs **NVIDIA**, certifique-se de ter os drivers proprietários instalados (`nvidia-smi` funcionando).
  - Em GPUs **AMD / Intel**, certifique-se de que o pacote `lm-sensors` está instalado e execute `sudo sensors-detect --auto`.

---

## 📄 Licença

Distribuído sob a licença [MIT](LICENSE).
