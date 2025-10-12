#!/usr/bin/env node
const fs = require('fs');
const {ethers} = require('ethers');
const yargs = require('yargs');

const argv = yargs
  .option('rpc-url', {type: 'string', demandOption: true})
  .option('private-key', {type: 'string', demandOption: true})
  .option('token', {type: 'string', demandOption: true})
  .option('csv', {type: 'string', default: 'data/initial_tgc_mints.csv'})
  .option('decimals', {type: 'number', default: 18})
  .option('batch', {type: 'number', default: 20})
  .argv;

const abi = [
  'function batchMint(bytes32 distributionId, address[] recipients, uint256[] amounts, bytes32[] reasons) external',
  'function decimals() view returns (uint8)'
];

async function main() {
  const provider = new ethers.providers.JsonRpcProvider(argv['rpc-url']);
  const wallet = new ethers.Wallet(argv['private-key'], provider);
  const token = new ethers.Contract(argv.token, abi, wallet);

  let decimals = argv.decimals;
  try { decimals = await token.decimals(); } catch (e) { console.log('could not read decimals, using', decimals); }

  const csv = fs.readFileSync(argv.csv, 'utf8');
  const lines = csv.split(/\r?\n/).filter(l => l.trim().length > 0);
  const entries = lines.map(l => {
    const [addr, amt, reason] = l.split(',').map(s => s.trim());
    const amount = ethers.BigNumber.from(amt).mul(ethers.BigNumber.from(10).pow(decimals));
    const r = reason && reason.startsWith('0x') ? reason : '0x' + Buffer.from(reason || '').toString('hex');
    return {addr, amount, reason: r};
  });

  // perform batched batchMint calls with distribution id derived from CSV content + offset
  for (let i=0;i<entries.length;i+=argv.batch) {
    const batch = entries.slice(i, i+argv.batch);
    const recipients = batch.map(b => b.addr);
    const amounts = batch.map(b => b.amount);
    const reasons = batch.map(b => {
      // ensure 0x-prefixed 32-byte hex; pad/truncate if necessary
      const r = b.reason.startsWith('0x') ? b.reason : '0x' + Buffer.from(b.reason || '').toString('hex');
      return r;
    });

    const distributionId = ethers.keccak256(ethers.toUtf8Bytes(argv.csv + ':' + i));
    console.log('batchMinting', recipients.length, 'recipients, distributionId', distributionId);
    const tx = await token.batchMint(distributionId, recipients, amounts, reasons);
    console.log('tx', tx.hash);
    await tx.wait();
  }
}

main().catch(err => { console.error(err); process.exit(1); });
