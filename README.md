# QR Clipboard App

クリップボードの内容をQRコードに変換するTauriアプリ

## 特徴

- ✨ ネイティブぼかし効果 (Windows: Acrylic, macOS: Vibrancy)
- 🎨 モダンでミニマルなUI
- 📱 小さいポップアップ式ウィンドウ
- 🔒 誤り訂正レベル: High
- 🖥️ クロスプラットフォーム対応

## セットアップ

### 前提条件

- Rust (1.70+)
- Node.js (18+)
- npm または yarn

### インストール

```bash
# 依存関係をインストール
npm install

# Tauriの依存関係をビルド
cd src-tauri
cargo build
cd ..
```

## 実行

### 開発モード

```bash
npm run tauri dev
```

### ビルド

```bash
npm run tauri build
```

ビルドされた実行ファイルは `src-tauri/target/release/` に生成されます。

## 使い方

1. アプリを起動
2. 自動的にクリップボードの内容を読み取り、QRコードを生成
3. 左側にQRコード、右側に元のテキストが表示されます

## トラブルシューティング

### Windowsでぼかしが効かない

Windows 11でアクリル効果が動作しない場合、システム設定で「透明効果」が有効になっているか確認してください。

### macOSで透明度が効かない

macOSの「ウィンドウの透明度を下げる」オプションが無効になっているか確認してください。

## 技術スタック

- **フロントエンド**: Vanilla JS, HTML, CSS
- **バックエンド**: Rust
- **フレームワーク**: Tauri 2.0
- **QRコード**: qrcode-rust
- **UI効果**: window-vibrancy

## ライセンス

MIT
