# rust_api_openai

Este projeto é uma API REST em Rust que utiliza Actix-web para expor um endpoint que envia perguntas para a API da OpenAI (ChatGPT) e retorna a resposta.

## Tecnologias utilizadas
- actix-web
- reqwest
- serde
- tokio

## Como fazer uma requisição

Faça um POST para `/ask` com o seguinte JSON:
```json
{
  "question": "Qual a capital da França?"
}
```

A resposta será:
```json
{
  "answer": "A capital da França é Paris.",
  "error": null
}
```
