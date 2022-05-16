### Korzystanie z algorytmów

1. Każdy algorytm jest opakowany w strukturę do inicjalizacji której wymagany jest jeden parametr: `<nazwa-algorytmu>Cfg` (move, nie referencja).
2. Każda struktura opakowująca algorytm wystawia jedną metodę (bez parametrów) uruchamiającą algorytm (proponuję `execute()`)

### Logowanie wyników 

Dla każdego algorytmu wystawimy traita `Probe` który wystawia metody pozwalające na zapisywanie informacji w istotnych etapach algorytmu, np.:

* on_best_fit_so_far()
* on_begin()
* on_new_generation()

itd.

Domyślna implementacja wypisuje na standardowe wyjście, do tego potrzebne będę jeszcze implementacje dla 
CSV, ewentualnie JSONa
