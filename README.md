# my_wallet
Projeto da disciplina DIM0547 - DESENVOLVIMENTO DE SISTEMAS WEB II

# Autor
Thales Gomes Moreira

# Descrição

O MyWallet é um sistema de gerenciamento de carteira virtual.O sistema permite ao usuário gerenciar seu saldo, por meio de operações de depósito, retirada e transferência e pagamento. Também é possível ter acesso às movimentações realizadas pelo usuário.

# Estórias de Usuário:
Como usuário cliente do sistema, eu desejo:
- Realizar meu cadastro, para utilizar a ferramenta;
- Consultar Meus dados, para verificar se estão corretos;
- Realizar depósito, para ter saldo para realizar operações;
- Realizar retirada, para usar o saldo fora da plataforma;
- Realizar transações, para pagar outros usuários, ou transferir saldo ;
- Consultar as operações realizadas, para manter controle sobre as operações;

Como usuário administrador do sistema, eu desejo:
- Alterar dados cadastrais de um usuário, manter os dados atualizados;
- Consultar as operações realizadas, para verificar histórico de operações;
- Desfazer uma transação, corrigir operações incorretas;

# Representação de dados:
Os dados da aplicação são armazenados nas tabelas, USERS e TRANSACTIONS. A tabela USERS contém os dados dos usuários registrados no sistema, já a tabela TRANSACTIONS registra as operações realizadas pelos usuários.

# Endpoints
- Register: Registra o usuário na tabela Users.
- Deposit: Atualiza o Balance(saldo) do usuário, realiza alterações na tabela USERS e na tabela TRANSACTIONS.
- Manifest: Retorna as operações em que o usuário foi envolvido, consulta a tabela TRANSACTIONS.
- Transfer: Realiza uma operação de transferência de saldo entre usuários. Insere registro na tabela TRANSACTIONS
- Withdraw: Faz uma retirada do Saldo do usuário. Atualiza o balance na tabela USERS e insere registro na tabela TRANSACTIONS
- Manage User: Faz alterações nos dados do usuário,realiza update na tabela USERS
- Manage Transfers: Permite atualizar as informações de transações ou inativar as mesmas, realiza alteração na tabela transactions e na tabela USERS.
