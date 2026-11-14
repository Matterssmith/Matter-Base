```
# Matter Base Architecture

## Purpose

Matter Base is the reusable foundation of the Matterssmith
ecosystem.

It provides infrastructure consumed by other repositories.

## Dependency direction

Applications and services depend on Matter Base.

Matter Base must never depend on individual applications.

```
Matter Ecosystem
       |
       v
  Applications
       |
       v
   Matter Base
       |
       +-- Core
       +-- Runtime
       +-- Context
       +-- Configuration
       +-- Logger
       +-- Services
       +-- Events
       +-- Modules
       +-- i18n