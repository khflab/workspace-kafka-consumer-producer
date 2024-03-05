# Workspace Consumer e Producer Kafka

Simples estrutura de workspace implementando um Hello World Rust e Kafka

## Passo a passo

Rode o docker compose para iniciar um Kafka de estudos e crie um tópico para testes.

Em seguida rode o consumidor

`cargo run --bin consumer topico-teste-criado`

Depois roder o produtor

`cargo run --bin producer topico-teste-criado`
