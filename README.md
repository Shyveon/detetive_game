# Jogo detetive
## Aluno: Lucas Dantas Romankiu

- tema
    - Horror

- Objetivo
    - Fazer um jogo simples, parecido com o enigma de einstein. Através de regras de inferência lógicas.

- Objetivo do jogo
    - Ajudar Alice à encontrar a porta correta para voltar a vida.

- História
    - Alice, sempre gostou de viver a vida sem limites. Em uma bela noite, voltando de mais uma noite de extravagâncias. Alice descobri que os freios do carro de seu pai que ele tanto cuidava e revisava não iriam ser bons o suficiente para salvar-lá, pobre carrinho, nunca teve chance. Alice bateu diretamente em um árvore, apesar de seus ferimentos não parecerem graves. A força da batida fez com que sua cabeça balançasse como um chicote e transformou seu cérebro em papinha...<br> Alice, acorda em uma sala escura iluminada por velas com chamas que não tremulam e nem queimam a parafína. Quando ela percebe um imenso olho aparece e lhe conta sobre a situação em que ela está.

|Porta|Certa ou errada?|
|:---:|:---:|
|Vermelha| Errada |
|Azul| Correta |
|Amarela| Errada |
|Verde| Errada|

Regras do jogo:
1. Maria se salvou.
2. Se Maria passou pela porta vermelha, então Pedro passou pela porta verde.
3. Pedro **não** passou pela porta verde.
4. Se Henrique passou pela porta verde, então Maria passou pela verde.
5. Se Maria passou pela porta amarela, então Henrique se salvou.
6. Maria não passou pela porta verde
7. Henrique não se salvou
8. Henrique passou pela porta verde e toda semente precisa de água para morrer
9. Se toda semenete precisa de água para morer e Tolomeo tem medo de decisões difíceis então Tolomeo ainda estão aqui.
10. Tolomeo tem medo de decisões difíceis.

Regras de inferência usadas:
    - Modus Tollen -> Maria não passou pela porta vermelha
    - Conjunção -> Maria não passou pela porta verde e henrique não se salvou
    - Dilema destrutivo -> Maria não passou pela porta amarela ou Henrique não passou pela porta verde
    - Silogismo disjuntivo -> Maria não passou pela porta amarela

- Logo como Maria se salvou e ela não passou pela porta vermelha, verde ou amarela. Então ela passou pela azul para se salvar.

---

Tabela com todas as regras de inferência

| Regra | Tautologia | Nome |
|:---:|:---:|:---:|
$p\\ \_\_\_\_\_\_ \\ \therefore (p \lor q)$| $(p \to (p \lor q))$| Adição
$p \wedge q\\ \_\_\_\_\_\_ \\ \therefore p$| $((p \wedge q) \to p)$| Simplificação
$p\\q\\ \_\_\_\_\_\_ \\ \therefore (p \wedge q)$| $((p) \wedge (q) \to (p \wedge q))$| Conjunção
$ p \to q\\ p\\ \_\_\_\_\_\_ \\ \therefore q $| $(((p \to q) \wedge p) \to q)$| Modus Ponens
$p \to q\\ \neg q\\ \_\_\_\_\_\_ \\ \therefore \neg p$| $(((p \to q) \wedge \neg q) \to \neg p)$| Modus Tollens
$p \lor q\\ \neg p \\ \_\_\_\_\_\_ \\ \therefore q$ | $(((p \lor q) \wedge \neg p) \to q)$ | Silogismo disjuntivo
$p \to q\\ q \to r\\ \_\_\_\_\_\_ \\ \therefore (p \to r)$| $(((p \to q) \wedge (q \to r)) \to (p \to r))$ | Silogismo hipotético
$p \to q\\ r \to s\\ p \lor r\\ \_\_\_\_\_\_ \\ \therefore (q \lor s)$| --- | Dilema construtivo
$p \to q\\ r \to s\\ \neg q \wedge \neg s\\ \_\_\_\_\_\_ \\ \therefore (\neg p \lor \neg r)$| --- | Dilema destrutivo
