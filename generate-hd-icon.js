// 生成高清图标的脚本
// 需要安装: npm install sharp png-to-ico

const sharp = require('sharp');
const pngToIco = require('png-to-ico');
const fs = require('fs');
const path = require('path');

async function generateIcons() {
  const sourceIcon = 'src-tauri/icons/app-icon.png';
  const iconsDir = 'src-tauri/icons';
  
  console.log('开始生成高清图标...');
  
  // 生成不同尺寸的 PNG（包括 256x256 用于高清任务栏图标）
  const sizes = [16, 32, 48, 64, 128, 256];
  const pngFiles = [];
  
  for (const size of sizes) {
    const outputFile = path.join(iconsDir, `temp-${size}.png`);
    await sharp(sourceIcon)
      .resize(size, size, {
        kernel: sharp.kernel.lanczos3 // 使用高质量缩放算法
      })
      .png({ quality: 100 })
      .toFile(outputFile);
    pngFiles.push(outputFile);
    console.log(`✓ 生成 ${size}x${size}.png`);
  }
  
  // 生成包含所有尺寸的 ICO 文件
  console.log('正在生成 icon.ico (包含 16/32/48/64/128/256 尺寸)...');
  const icoBuffer = await pngToIco(pngFiles);
  fs.writeFileSync(path.join(iconsDir, 'icon.ico'), icoBuffer);
  console.log('✓ 生成 icon.ico');
  
  // 清理临时文件
  console.log('清理临时文件...');
  for (const file of pngFiles) {
    fs.unlinkSync(file);
  }
  
  console.log('\n✅ 图标生成完成！icon.ico 现在包含高清 256x256 层级');
  console.log('重新编译应用后，任务栏图标将显示清晰');
}

generateIcons().catch(console.error);
