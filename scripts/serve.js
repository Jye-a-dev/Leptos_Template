const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');

let port = 5000;
const envPath = path.resolve(__dirname, '..', '.env');
if (fs.existsSync(envPath)) {
  const envContent = fs.readFileSync(envPath, 'utf8');
  const match = envContent.match(/^PORT\s*=\s*(\d+)/m);
  if (match) {
    port = parseInt(match[1], 10);
  }
}

const isRelease = process.argv.includes('--release');
const args = ['serve', '--port', String(port)];
if (isRelease) {
  args.splice(1, 0, '--release');
}

const child = spawn('trunk', args, {
  stdio: 'inherit',
  shell: true,
  cwd: path.resolve(__dirname, '..')
});

child.on('exit', (code) => {
  process.exit(code || 0);
});
