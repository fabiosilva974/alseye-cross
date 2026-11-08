#!/usr/bin/env bash
# Script de Instalação do ALSEYE BEM Controller no Linux
set -e

echo "=== Instalador ALSEYE BEM Controller (Linux) ==="

# 1. Detectar Gerenciador de Pacotes e Instalar Dependências
if command -v apt &> /dev/null; then
    echo "[+] Detectado Ubuntu/Debian. Instalando dependências..."
    sudo apt update
    sudo apt install -y build-essential pkg-config libudev-dev libhidapi-dev lm-sensors
elif command -v dnf &> /dev/null; then
    echo "[+] Detectado Fedora/RHEL. Instalando dependências..."
    sudo dnf install -y gcc gcc-c++ make pkgconf-pkg-config systemd-devel hidapi-devel lm_sensors
elif command -v pacman &> /dev/null; then
    echo "[+] Detectado Arch Linux. Instalando dependências..."
    sudo pacman -S --needed --noconfirm base-devel pkgconf systemd hidapi lm_sensors
else
    echo "[!] Gerenciador de pacotes não reconhecido. Certifique-se de ter Rust, pkg-config, libudev e hidapi instalados."
fi

# 2. Configurar Regras UDEV para Acesso USB sem Root
echo "[+] Configurando regras UDEV para o dispositivo USB HID (VID: 0x5131, PID: 0x2007)..."
sudo cp "$(dirname "$0")/99-alseye.rules" /etc/udev/rules.d/99-alseye.rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Adicionar usuário aos grupos dialout/plugdev
USER_NAME="${SUDO_USER:-$USER}"
sudo usermod -aG plugdev "$USER_NAME" 2>/dev/null || true
sudo usermod -aG dialout "$USER_NAME" 2>/dev/null || true

# 3. Compilar em Release
echo "[+] Compilando binário em modo Release..."
cd "$(dirname "$0")/.."
cargo build --release

# 4. Instalar Executável no Sistema
echo "[+] Instalando binário em /usr/local/bin/alseye-cross..."
sudo cp target/release/alseye-cross /usr/local/bin/
sudo chmod +x /usr/local/bin/alseye-cross

# 5. Instalar Serviço do Usuário Systemd (opcional)
mkdir -p "$HOME/.config/systemd/user/"
cp "$(dirname "$0")/alseye-cross.service" "$HOME/.config/systemd/user/"
systemctl --user daemon-reload

echo ""
echo "=== Instalação Concluída com Sucesso! ==="
echo "Para executar agora:"
echo "  alseye-cross"
echo ""
echo "Para habilitar inicialização automática no login:"
echo "  systemctl --user enable --now alseye-cross.service"
echo ""
echo "Nota: Se o seu usuário acabou de ser adicionado aos grupos, pode ser necessário reiniciar a sessão (logout/login)."

