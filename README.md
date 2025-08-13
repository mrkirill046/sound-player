# 🎧 Sound Player

[![Status](https://img.shields.io/badge/status-stable-green?style=flat-square&logo=appveyor)](https://github.com/mrkirill046/sound-player)
[![Built with Tauri](https://img.shields.io/badge/built%20with-tauri-blueviolet?logo=tauri&style=flat-square)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-powered-ff3e00?logo=svelte&logoColor=white&style=flat-square)](https://svelte.dev/)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)
[![Latest Release](https://img.shields.io/github/v/release/mrkirill046/sound-player?style=flat-square&logo=github)](https://github.com/mrkirill046/sound-player/releases/latest)

> Мощный, кроссплатформенный и лёгкий музыкальный плеер, построенный на **Rust**, **Tauri**, **Svelte** и **Skeleton UI**.  
> Минимализм. Скорость. Красота.

---

## 🚀 Возможности

- ⚡ **Невероятная производительность** благодаря нативному Rust + Tauri
- 🎨 **Интерфейс нового поколения** с помощью [Svelte](https://svelte.dev) + [Skeleton UI](https://www.skeleton.dev)
- 🌙 Поддержка **тёмной/светлой темы**
- 🎵 Локальное воспроизведение треков (MP3, WAV и др.)
- 📁 Drag & Drop, плейлисты, прогрессбар, горячие клавиши

---

## 🛠️ Стек технологий

| Технология     | Назначение                            |
|----------------|----------------------------------------|
| [Rust](https://www.rust-lang.org/)     | Ядро и backend логика плеера       |
| [Tauri](https://tauri.app/)           | Обёртка для кроссплатформенного GUI |
| [SvelteKit](https://kit.svelte.dev/) | Фронтенд-фреймворк нового поколения |
| [TypeScript](https://www.typescriptlang.org/) | Безопасность и типизация           |
| [Skeleton UI](https://www.skeleton.dev/) | Tailwind UI-компоненты с темами     |

---

## 📦 Установка и запуск

> Требуется: **Rust**, **Node.js**, **bun**

### 1. Установка зависимостей

```bash
bun install
````

### 2. Запуск в режиме разработки

```bash
bun tauri dev
```

### 3. Сборка проекта

```bash
bun tauri build
```

---

## 🖼️ Скриншоты

<details>
    <summary>Главное окно</summary>
    <img src="assets/screenshot-1.png"/>
    <img src="assets/screenshot-2.png"/>
</details>
<details>
    <summary>Настройки</summary>
    <img src="assets/screenshot-3.png"/>
</details>

---

## 📁 Структура проекта

```text
sound-player/
├── src/                  # Фронтенд: Svelte + Skeleton
│   ├── app.html
│   └── routes/           # Роуты приложения
│       ├── +layout.ts
│       └── +page.svelte

├── src-tauri/            # Backend на Rust + Tauri
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/              # Точка входа в Rust-приложение
│   │   └── main.rs
│   └── icons/            # Иконки для всех платформ

├── static/               # Статические ассеты
├── package.json          # Node.js зависимости
├── svelte.config.js      # Конфигурация SvelteKit
├── vite.config.js        # Конфигурация Vite
├── tsconfig.json         # TypeScript конфиг
└── README.md             # Документация проекта
```

---

## 🧩 Планы на будущее

- [x] Горячие клавиши и медиа-кнопки
- [x] Настройки тем
- [ ] Плагины и расширения

---

## 📄 Лицензия

MIT — делай с этим что хочешь, но не забывай об авторах. 😉

---

## 🧠 Автор

Разработка: [Кирилл aka Kazuha](https://kazuha046.qwy-games.ru)
❤️ Идея, код, дизайн, вдохновение — всё сам.

---

> *“Приложения не должны быть жирными. Они должны быть быстрыми, красивыми и дышать.”*
> — ты
