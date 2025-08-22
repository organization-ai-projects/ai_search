# Journal d'expérimentations — Activations custom MNIST

Ce fichier recense les essais d'activations (ou compositions) sur le réseau minimal MNIST, avec les paramètres principaux et le résultat observé.

---

## Format
- **Activation** : description ou code Rust minimal
- **Paramètres** : batch, lr, epochs, etc.
- **Résultat** : collapse, apprentissage, accuracy, etc.
- **Commentaires** : observations, idées, remarques

---

## Essais

### 1. Sigmoid (de base)
- **Activation** : sigmoid
- **Résultat** : apprentissage correct, baseline MNIST

### 2. Fractale seule (profondeur 3)
- **Activation** : fractal_activation(x, 3)
- **Résultat** : collapse, une seule classe prédite

### 3. Mycélienne seule
- **Activation** : mycelium_activation(x)
- **Résultat** : collapse, une seule classe prédite

### 4. Logistique normalisée
- **Activation** : logistic_activation(x)
- **Résultat** : collapse, une seule classe prédite

### 5. Pulsative
- **Activation** : pulsative_activation(x)
- **Résultat** : collapse, une seule classe prédite

### 6. Logistique → fractale (profondeur 3)
- **Activation** : fractal_activation(logistic_activation(x), 3)
- **Résultat** : collapse, une seule classe prédite

### 7. Fractale (profondeur 3) → mycélienne
- **Activation** : mycelium_activation(fractal_activation(x, 3))
- **Résultat** : collapse, une seule classe prédite

### 8. Logistique → trou noir doux (gaussienne)
- **Activation** : soft_blackhole_activation(logistic_activation(x))
- **Résultat** : collapse, toutes les sorties nulles

---

**À compléter à chaque nouvel essai !**

### 10. Dynamique temporelle implicite (mémoire récurrente)
- **Activation** : tanh standard, mais chaque entrée reçoit deux features supplémentaires : t (progression temporelle) et y_t = tanh(mean(x) + β·t + α·y_{t-1})
- **Paramètres** : batch=16, lr=0.1, epochs=10, dataset=mnist, n_samples=100, input_size augmenté de 2 (t, y_t), β=2.0, α=0.8
- **Résultat** : accuracy finale ~40% après 1000 epochs (n_samples=100), diversité accrue (prédictions sur 0, 2, 4, 6, 8), mais collapse partiel persistant (majorité sur 8, proba ~0.80-0.93)
- **Commentaires** : y_t simule une mémoire temporelle simple, injectée comme feature. À observer si le réseau exploite cette dynamique ou si le collapse persiste.

### 9. Activation dépendant du temps (tanh(x + βt))
- **Activation** : `|x| (x + β·t).tanh()` où t = feature ajoutée à chaque entrée, β=2.0
- **Paramètres** : batch=16, lr=0.1, epochs=10, dataset=mnist, n_samples=100, input_size augmenté de 1 (feature t)
- **Résultat** : collapse, une seule classe prédite (classe 8, proba ~0.70), accuracy ~8%
- **Commentaires** : Ajouter t comme feature et l'utiliser dans l'activation ne permet pas d'apprentissage sur MNIST dans ce setup minimal. Le temps n'est pas exploité de façon utile par le réseau. À explorer : manipuler t de façon non-linéaire, ou l'utiliser comme modulation dynamique (gating, scaling, etc.).

Idées à tester :
- Compositions plus douces (tanh, relu, etc.)
- Paramètres différents (profondeur fractale, r logistique, etc.)
- Visualisation des activations intermédiaires
- ...
