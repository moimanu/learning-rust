# Exercícios de Programação Dinâmica (Rust)

Este repositório contém a resolução de quatro problemas clássicos de Programação Dinâmica implementados em Rust com abordagem *bottom-up* e otimização de memória. Para validar todas as implementações, execute `cargo test` na raiz do projeto.

---

## 1. Escada com Passos de 1, 2 ou 3 Degraus

A implementação em [`src/staircase.rs`](src/staircase.rs) determina o número de maneiras distintas de alcançar o topo de uma escada de $n$ degraus subindo 1, 2 ou 3 degraus por vez. O estado `dp[i]` representa o total de combinações para atingir o degrau $i$, partindo dos casos-base `dp[0] = 1`, `dp[1] = 1` e `dp[2] = 2`. Como qualquer degrau $i \ge 3$ é alcançado diretamente a partir dos três degraus anteriores, a transição de estados segue a recorrência $dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3]$. A solução *bottom-up* mantém apenas três variáveis escalares para os estados imediatamente anteriores, operando em tempo $O(n)$ e espaço $O(1)$.

---

## 2. Maior Soma sem Elementos Vizinhos

O algoritmo em [`src/non_adjacent_sum.rs`](src/non_adjacent_sum.rs) calcula a maior soma possível de um subconjunto de elementos sem selecionar posições adjacentes. O estado `dp[i]` armazena a soma máxima acumulada até o índice $i$, tendo como base `dp[0] = nums[0]` e `dp[1] = max(nums[0], nums[1])`. A cada passo $i \ge 2$, decide-se entre ignorar o elemento atual (mantendo `dp[i - 1]`) ou somá-lo ao melhor resultado não adjacente (`dp[i - 2] + nums[i]`), resultando na recorrência $dp[i] = \max(dp[i - 1],\, dp[i - 2] + nums[i])$. A implementação utiliza duas variáveis para rastrear os estados anteriores, atingindo complexidade de tempo $O(n)$ e espaço $O(1)$.

---

## 3. Caminho de Custo Mínimo em Matriz

O módulo [`src/min_path_sum.rs`](src/min_path_sum.rs) encontra o menor custo acumulado para percorrer uma matriz do canto superior esquerdo `(0, 0)` ao inferior direito `(m - 1, n - 1)` com movimentos restritos para a direita e para baixo. O estado `dp[i][j]` define o custo mínimo até a célula `(i, j)`. A origem inicia em `dp[0][0] = grid[0][0]`, enquanto a primeira linha e a primeira coluna acumulam os custos de seus únicos vizinhos válidos (esquerdo e superior, respectivamente). Para as demais células, aplica-se a recorrência $dp[i][j] = grid[i][j] + \min(dp[i - 1][j],\, dp[i][j - 1])$. O espaço é reduzido de uma matriz completa para um único vetor de tamanho $n$ atualizado linha a linha, garantindo tempo $O(m \times n)$ e espaço $O(n)$.

---

## 4. Troco com Número Mínimo de Moedas

A solução em [`src/coin_change.rs`](src/coin_change.rs) calcula a quantidade mínima de moedas necessárias para compor um valor alvo `target`. O estado `dp[x]` representa o menor número de moedas para formar a quantia $x$, iniciando com `dp[0] = 0` e os demais estados em infinito (`usize::MAX`) para indicar valores ainda inacessíveis. Na construção *bottom-up*, itera-se de $1$ até o valor alvo testando cada moeda disponível $c \le x$ através da relação $dp[x] = \min(dp[x],\, 1 + dp[x - c])$. Se o estado final `dp[target]` permanecer infinito após o processamento, o valor é impossível de ser formado e a função retorna `None`. O algoritmo executa em tempo $O(\text{valor} \times |moedas|)$ e espaço $O(\text{valor})$.