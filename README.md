# Projet de session - 0SW - Raphaël Thériault

## Utilisation

La configuration des différentes valeurs se fait par ligne de commande. Utilisez le flag `--help` pour voir toutes les options disponibles.

Les graphiques sont générés lors de la fermeture du programme dans un dossier `stats` relatif au dossier d'où le programme a été lancé.

### Commandes

-   `[SPACE]` - Play/Pause
-   `[UP]` - Augmenter la vitesse de simulation (itérations par frame)
-   `[DOWN]`- Diminuer la vitesse de simulation (itérations par frame)
-   `[D]` - Activer/Désactiver la vue détaillée (champs de vision et direction)

## Déroulement

Pour chaque frame affiché à l'écran, le processus suivant est appliqué pour chaque créature un nombre de fois égal au multiplicateur de vitesse.

1. Recherche de nourriture dans le champ de vision (autres créatures plus petites ou mortes pour les carnivores, objets pour les herbivores)
2. Application d'un vecteur de braquage vers la nourriture la plus proche, si il y en a
3. Recherche de prédateurs dans le champ de vision
4. Application d'un vecteur de braquage s'éloignant de chaque prédateur de force relative à la distance avec le prédateur
5. Déplacement selon le vecteur résultant et la vitesse de la créature et diminution de l'énergie selon l'endurance
6. Détection de collision avec la nourriture
7. Si il y a collision, la nourriture est consommée par la créature et son énergie augmente (les carnivores obtiennent un bonus en mangeant des créatures vivantes en "volant" leur énergie restante)

## Caractéristiques

### Restreintes

Le total de ces statistiques doit se trouver dans une certaine marge pour éviter que les créatures deviennent "parfaites" avec le temps.

-   `speed` - Vitesse relative à laquelle la créature se déplace
-   `stamina` - Valeur qui détermine la quantitée relative d'énergie que la créature doit dépenser pour se déplacer d'une certaine distance
-   `fov` - Champ de vision dans lequel la créature peut repérer d'autres créatures ou objets
-   `size` - Taille de la créature qui détermine quelles autres créatures peuvent intéragir avec elle

### Arbitraires

-   `diet` - Détermine l'alimentation de la créature, soit herbivore ou carnivore

## Compilation

Il faut avoir Rust installé pour compiler, le moyen le plus simple et recommandé est [rustup](https://rustup.rs/).

L'éxécutable est placé dans `target/release/`.

1. `cargo install cargo-vcpkg`
2. `cargo vcpkg build`
3. `cargo run --release` ou `cargo build --release`

## Références

### Wikipedia

-   [Evolutionary algorithm](https://en.m.wikipedia.org/wiki/Evolutionary_algorithm) - Algorithme inspiré de l'évolution biologique servant à résoudre un problème d'optimisation
-   [Evolutionary programming](https://en.m.wikipedia.org/wiki/Evolutionary_programming) - Sous-genre de l'evolutionary algorithm où la structure du programme est fixe mais différentes valeurs numériques peuvent évoluer
-   [Genetic algorithm](https://en.m.wikipedia.org/wiki/Genetic_algorithm) - Sous-genre de l'evolutionary algorithm qui imite le principe de sélection naturalle en partant d'une population de base aléatoire
-   [Selection](<https://en.m.wikipedia.org/wiki/Selection_(genetic_algorithm)>) - Processus servant à déterminer quels individus dans une population se reproduisent
-   [Crossover](<https://en.m.wikipedia.org/wiki/Crossover_(genetic_algorithm)>) - Opération génétique qui sert à déterminer les caractéristiques d'un individu depuis celles de ses parents
-   [Mutation](<https://en.m.wikipedia.org/wiki/Mutation_(genetic_algorithm)>) - Opération génétique qui sert à maintenir un seuil minimum de diversité dans un population

### Videos

-   [Primer - Simulating Natural Selection](https://www.youtube.com/watch?v=0ZGbIKd0XrM)
-   [Primer - Simulating the Evolution of Aggression](https://www.youtube.com/watch?v=YNMkADpvO4w)
-   [carykh - Evolution Simulators](https://www.youtube.com/playlist?list=PLrUdxfaFpuuK0rj55Rhc187Tn9vvxck7t)
