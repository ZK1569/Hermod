# Projet Hermod: _encrypted communication cli_ <br> 8INF857 - UQAC

The aim of this project is to provide encrypted messaging between 2 machines

## Optimum execution scenario

1. Server starts

2. Communication parameters are loaded.

   - Defining the port to use

3. Loop until interrupted

   The server is listening on port 8080 (default), waiting for a client to connect.

   There are several types of interaction between a worker and the server

   1. Server and client exchange messages

      This message is of type `CommunicationText` and includes

      - Its identification

   2. Server and client exchange files
      TODO:

   3. Server and client exchange certificats
      TODO:

## Cryptography

TODO:

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
