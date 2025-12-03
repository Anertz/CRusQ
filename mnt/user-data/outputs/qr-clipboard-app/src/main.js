import { invoke } from '@tauri-apps/api/core';

async function generateQR() {
  const qrPlaceholder = document.getElementById('qr-placeholder');
  const qrImage = document.getElementById('qr-image');
  const clipboardText = document.getElementById('clipboard-text');
  const errorMessage = document.getElementById('error-message');

  try {
    // Rustのコマンドを呼び出し
    const result = await invoke('generate_qr_from_clipboard');
    
    // QRコード画像を表示
    qrImage.src = result.image;
    qrImage.style.display = 'block';
    qrPlaceholder.style.display = 'none';
    
    // クリップボードのテキストを表示
    clipboardText.textContent = result.text;
    
    // エラーメッセージを非表示
    errorMessage.style.display = 'none';
  } catch (error) {
    console.error('Error generating QR code:', error);
    
    // エラー表示
    errorMessage.textContent = `Error: ${error}`;
    errorMessage.style.display = 'block';
    
    // プレースホルダーを表示
    qrPlaceholder.style.display = 'flex';
    qrImage.style.display = 'none';
    
    // エラーメッセージをテキストエリアにも表示
    clipboardText.textContent = 'Failed to read clipboard or generate QR code.';
  }
}

// ページ読み込み時に実行
window.addEventListener('DOMContentLoaded', () => {
  generateQR();
});

// 定期的にクリップボードをチェック (オプション)
// setInterval(generateQR, 3000);
