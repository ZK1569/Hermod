# Projet Hermod: _encrypted communication cli_ <br> 8INF857 - UQAC256

The aim of this project is to provide encrypted messaging between 2 machines

## Optimal execution scenario

1. Server starts

2. The server chooses a password for the conversation
   
   According to this password, a symmetrical encryption key is derived

4. Client send hash
   
   After the connection has been established, the client sends a hash (sha-512) of the password entered by the user.
   The client has 3 tries to send the right password hash

6. Established connection
   
   The connection is established between the client and the server
   
   Data is sent in encrypted form
   
   There are several types of interaction between a worker and the server

   1. Server and client exchange messages

      This message is of type `CommunicationText`, the message is in `data`

   2. Server and client exchange files
      TODO:

   3. Server and client exchange certificats
      TODO:

## Cryptography

1. Password Hashing with [_SHA-512_](https://en.wikipedia.org/wiki/SHA-2)
   
   We use SHA-512 to compute the hash of the password before sending it securely over the network.
   By choosing SHA-512 over SHA-256, we increase the computational complexity,
   enhancing resistance against brute-force attacks.
   The additional processing time is negligible in our context,
   so increasing it does not impact overall performance significantly.
   Even if there are slowdowns during this process, it is acceptable for our application.

3. Symmetric Encryption with [_AES-256-CBC_](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)
   
   For symmetric encryption of the data content, we utilize AES-256-CBC.
   Only the data content is encrypted; the communication JSON remains unencrypted.
   We choose a 256-bit key size to balance encryption speed and security effectively.
   The CBC (Cipher Block Chaining) mode is employed to prevent data leaks and enhance security,
   which is particularly crucial when transferring files.

## The exchange protocol

Messages are exchanged over a TCP stream in the form of a sequence of bytes,
the result of serializing a hybrid structure combining a JSON description and binary data.

Indeed, to optimize the transfer of file or text information
(which can be voluminous and for which you don't want to commit rounding)

All messages are of the form:

| Total message size         | JSON message size          | JSON message     | Data...       |
| -------------------------- | -------------------------- | ---------------- | ------------- |
| (u32 encodé en Big Endian) | (u32 encodé en Big Endian) | (encodé en utf8) | (binary data) |

The _Data_ section is therefore made up of (Total message size) - (JSON message size) bytes and contains all the data corresponding to the file or text.
data corresponding to the file or text

### Message sequencing
<img width="1371" alt="Doc" src="https://github.com/user-attachments/assets/6967f959-8224-4b08-a04d-b0af2d50569a">

### Description des messages

All these messages are transmitted in the form of a [JSON](https://fr.wikipedia.org/wiki/JavaScript_Object_Notation) serialization

| Name of message            | Message fields                  | Exemple                                                |
| -------------------------- | ------------------------------- | ------------------------------------------------------ |
| `CommunicationText`        |                              | `{"CommunicationText: {}}`                             |
| `CommunicationCertificate` | `TODO:`                           | `TODO:`                                                  |
| `CommunicationFile`        | `TODO:`                           | `TODO:`                                                  |
| `CommunicationPassword`    | `password_state: PasswordState` | `{"CommunicationPasword":{"password_state":"Correct"}` |

### Complementary types

| Type name       | Description of type                                                |
| --------------- | ------------------------------------------------------------------ |
| `PasswordState` | `"Submition"` OR<br/>`"Correct"` OR <br/> `"Incorrect"` OR <br/>`"Failed"` |
