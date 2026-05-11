# Module-9-Software-Architectures-Publisher

## Pertanyaan Tutorial

### a. Berapa banyak data yang akan dikirim oleh program publisher Anda ke message broker dalam satu kali jalan?
Program publisher akan mengirimkan sebanyak **5 pesan** (dengan `user_id` 1 hingga 5) ke message broker dalam satu kali eksekusi.

### b. URL: "amqp://guest:guest@localhost:5672" sama dengan yang ada di program subscriber, apa artinya?
Hal ini berarti kedua program tersebut terhubung ke **instansi message broker (RabbitMQ) yang sama** yang sedang berjalan di mesin lokal (`localhost`) menggunakan port standar AMQP (`5672`). Kesamaan URL ini memungkinkan publisher untuk mengirim pesan ke broker yang nantinya akan diterima oleh subscriber melalui queue yang telah ditentukan.

## Dokumentasi Pengujian
![Monitoring Chart](./assets/message-rates.png)

### Penjelasan Lonjakan (Spikes)
Lonjakan (spikes) pada grafik **Message rates** di atas terjadi saat program Publisher dijalankan. Hal ini karena Publisher mengirimkan 5 pesan sekaligus dalam waktu yang sangat singkat ke message broker. Grafik tersebut memantau aktivitas pengiriman pesan secara *real-time*, sehingga setiap kali eksekusi `cargo run` dilakukan pada Publisher, grafik akan menunjukkan puncak (peak) aktivitas.
