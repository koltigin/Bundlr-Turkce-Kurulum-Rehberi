# Bundlr Network Türkçe Node Kurulum Rehberi
![image](https://user-images.githubusercontent.com/102043225/182342392-a15ecdb4-3aca-4cd0-bae8-f9ad4239ccb3.png)
##  Sistem Gereksinimleri
* 4vCPU
* 8GB RAM
* 250GB SSD

## Sistemi Güncelleme
```shell
sudo apt update && sudo apt upgrade -y
```

## Gerekli Kütüphanelerin Kurulması
```shell
apt-get install cargo git make wget curl clang snapd pkg-config libssl-dev build-essential libpq-dev git jq openssl ocl-icd-opencl-dev ncdu libgomp1 bsdmainutils htop -y < "/dev/null"
```

## Docker Kurulumu
```sh
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
```

## Docker Compose Kurulumu
```sh
VER=$(curl -s https://api.github.com/repos/docker/compose/releases/latest | grep tag_name | cut -d '"' -f 4)
curl -L "https://github.com/docker/compose/releases/download/"$VER"/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose
docker-compose --version
```

## Bundlr İndirilmesi
```shell
git clone --recurse-submodules https://github.com/Bundlr-Network/validator-rust.git
```

## Cüzdan Oluşturma
[Arweave wallet](https://arweave.app/) adresine giderek bir cüzdan oluşturun. Cüzdan oluşturduktan sonra size `.json` uzantılı bir dosya verecektir. Bu dosyanınadını `wallet.json` olarak değiştirdikten sonra bu dosyayı sunucunuzda `validator-rust` klasörünün içine kopyalayın.

Cüzdan oluşturma ile ilgili detayları öğrenmek için [Arweave Dökümantasyon](https://docs.arweave.org/info/wallets/arweave-web-extension-wallet) adresini ziyaret edebilirsiniz.

## ENV Dosyası Oluşturma

```environment
tee $HOME/validator-rust/.env > /dev/null <<EOF
PORT="2023"
VALIDATOR_KEY="~/validator-rust/wallet.json"
BUNDLER_URL="https://testnet1.bundlr.network" 
GW_CONTRACT="RkinCLBlY4L5GZFv8gCFcrygTyd5Xm91CzKlR6qxhKA"  
GW_WALLET="~/validator-rust/wallet.json"
GW_ARWEAVE="https://arweave.testnet1.bundlr.network"
EOF
```

## Docker'ı Başlatma

Kurulum yaklaşık 10 dakika kadar sürebilir. Bağlantı kesilmesi ihtimalini göz önüne alarak önce screen oluşturuyoruz.
```shell
screen -S Bundlr
```
Daha sonra docker'ı başlatıyoruz.
```sh
cd ~/validator-rust && docker-compose up -d
```

## Depoyu Güncelleme
```shell
git pull origin master
```

## Docker Çalıştırma
```shell
docker-compose build
```

## Node.js kurulumu
```shell
source ~/.bashrc
sudo apt-get install snapd
sudo snap install node --channel=16/stable --classic
```

## NPM Güncelleme
```shell
npm install -g npm@8.15.0
```

## CLI Yükleme
```shell
npm i -g @bundlr-network/testnet-cli
source $HOME/.profile
```

## Faucet
[Bundlr Faucet](https://bundlr.network/faucet) adresine giderek token alıyoruz.

Aşağıdaki kod ile bakiyemizi sorguluyoruz.
`CUZDAN_ADRESINIZ` bölümüne cüzdan adresinizi yazınız.
```shell
testnet-cli balance CUZDAN_ADRESINIZ
```

## Validator Oluşturma
```shell
testnet-cli join RkinCLBlY4L5GZFv8gCFcrygTyd5Xm91CzKlR6qxhKA -w ~/validator-rust/wallet.json ~/ -u  http://$(curl icanhazip.com):4444 -s 25000000000000
```


## Validator Kaydınızı Kontrol Etme
`CUZDAN_ADRESINIZ` bölümüne cüzdan adresinizi yazınız.
```shell
npx @bundlr-network/testnet-cli@latest check RkinCLBlY4L5GZFv8gCFcrygTyd5Xm91CzKlR6qxhKA CUZDAN_ADRESINIZ
```

## Explorer

Bu [adresten](https://bundlr.network/explorer) kontrol edebilirsiniz.

## Faydalı Komutlar

### Logları Başlatma
```sh
cd ~/validator-rust && docker-compose logs --tail=100 -f
```

### Node'u Başlatma
```sh
npm i -g @bundlr-network/testnet-cli
```
