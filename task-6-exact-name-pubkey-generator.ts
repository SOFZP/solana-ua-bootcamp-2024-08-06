import { Keypair } from "@solana/web3.js";
import * as readline from 'readline';

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

// Функція для отримання вводу від користувача
const question = (query: string): Promise<string> => new Promise(resolve => rl.question(query, resolve));

// Головна функція
(async () => {
    const prefix = await question("Введіть ім'я для пошуку (макс. 4 символи): ");
    if (prefix.length > 4) {
        console.log("❌ Помилка: ім'я не може перевищувати 4 символи.");
        process.exit(1);
    }

    const caseSensitiveAnswer = await question("Чи чутливий до регістру пошук? (yes/no): ");
    const caseSensitive = caseSensitiveAnswer.toLowerCase() === 'yes';

    let keypair: Keypair;
    let publicKey: string;
    let attempts = 0;
    const startTime = Date.now();

    console.log("🔍 Починається пошук...");

    while (true) {
        attempts++;
        keypair = Keypair.generate();
        publicKey = keypair.publicKey.toBase58();

        const target = caseSensitive ? prefix : prefix.toLowerCase();
        const candidate = caseSensitive ? publicKey.slice(0, prefix.length) : publicKey.slice(0, prefix.length).toLowerCase();

        if (candidate === target) {
            break;
        }

        if (attempts % 100000 === 0) {
            console.log(`🔄 Перебрано ${attempts} ключів...`);
        }
    }

    const endTime = Date.now();
    const duration = (endTime - startTime) / 1000;

    console.log(`✅ Знайдено!`);
    console.log(`🔑 Публічний ключ: ${publicKey}`);
    console.log(`🔐 Приватний ключ: ${keypair.secretKey}`);
    console.log(`⏳ Час перебору: ${duration} секунд`);
    console.log(`🔢 Кількість переборів: ${attempts}`);

    rl.close();
})();
