# Software

-------------------------------------

Analyse:
• Sammeln und validieren von funktionalen Anforderungen für ein Softwaresystem
mit einem Stakeholder gemäß einer Standardmethodik und Erstellen von
Akzeptanzkriterien
• Durchführung einer Analyse der Funktionalität eines bestehenden
Softwaresystems oder einer Komponente, um die Möglichkeiten und
Unmöglichkeiten einer Anpassung zu ermitteln
• Analyse, ob ein bestimmter Datensatz Informationen für eine bestimmte
Anwendung liefert
Design:
• Entwerfen eines Designs für ein Softwaresystem einschließlich einer Datenbank
unter Verwendung von Modellierungstechniken gemäß einer Standardmethode.
• Erstellen von Testskripten für Endbenutzer-/Abnahmetests.
Umsetzung:
• Auf strukturierte Weise ein einfaches Softwaresystem entwickeln, testen und
bereitstellen, das mit strukturierten Daten arbeitet und grundlegende
Qualitätsanforderungen erfüllt
• (Automatisierte) Komponententests erstellen und durchführen

-------------------------------------

## Stakeholder

## Functional Requirements

Http 1.1 specification

## Non-Functional Requirements

load

## webserver

Implementation of the webserver and terminal application 

[TODO]: // diagram

### server

Accepts TCP connection and distributes to thread pool.
Orchestrates connection handling from request to response.
Logs events and resource usage to telemetry channel.

#### Thread pool

The server is setup into to following threads:
- Acceptor (1) 
  - Listens to TCP listener and accepts connections
  - Connection are sent to the dispatcher via channel
- Dispatcher (1)
  - Receives connections from acceptor and distributes to workers with round-robin strategy 
- Workers (Configurable, default 4)
  - Handle the connection from dispatcher and handles it to completion until taking the next connection 

This setup is to allow the acceptor to always accept requests while the dispatcher works out the strategy to distribute the workload. 
In the future the round-robin setup could be replaced with a different strategy more fit for the use-case.
The implementation is not asynchronous and quite expensive. 
To minimize the number of external libraries something like Tokio was deliberately not used, but would be advantages in this application.
The channels distributing the connections are synchronous and have a limit configured (default 1024). 
This setup creates backpressure and starts rejecting connections if the system is overloaded. 

### parser

Connections get parsed into a HTTP Request model.
The connection is read in incremental steps to validate each part bevor continuing.
First the request line gets read and checked if it is valid / supported. 
The headers get read and the leftover bytes of the head get saved in the body.
To conserve resources the body is only read when necessary but could be handled cleaner in the event of reverse proxy handling.
Requests get pared in the same model, sharing implementation for headers and body. 

### validation

The request is parsed into the right model and gets validated for specification correctness or invalid values. 

### routing

The request URI is checked against the configuration to find a matching route. 
There are 3 different strategies matching the route: exact, prefix and regex.

### handler

Given the route the action gets interpreted and handled. 
The 3 supported actions are: fixed directory serving, redirect and reverse proxy.

### http

Defines Request and Response models representing the HTTP Specification

### configuration

Models and parses the configuration file. 
The external model is represented by primitives and later parsed into the internal arithmetic model.

### telemetry

Model to submit events and usage for observability 

### tui

Visualizes telemetry over time to observer load. 

## bench

A test suite to assure functional and non-functional requirements.
For ease of use a simple terminal user interface presents and reports test suites. 
Tests are extendable and generic to allow different types of testing.  


