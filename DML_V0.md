# DML - Dolt markup language

DML est un language de balisage permettant la création de document structuré
à partir d'un fichier texte.

## Contraintes

Le language est conçus afin de répondre aux exigences suivantes:

- Simple et rapide à écrire afin d'augmenter la productivité.
- Simple à traiter afin de réduire la complexité du code du parser et ainsi
  diminué le risque de bug.
- Permettre facilement l'usage d'élément intéractif
- Extensible facilement
- Génération de fichier HTML5 et PDF

## Source d'inspiration

Le language s'inspière du langage reStructuredText et du langage Makdown

## Principes

### Les blocs

Le bloc est l'unité de base permettant la structuration d'un text. Il y a deux
type de bloc, les blocs annonymes, qui sont des lignes de text contigue et les
blocs nommés qui ce trouvent entre deux lignes commençant par "--- ". Le nom
du bloc doit être écrit après les trois tirets et l'espace ouvrant le bloc.

Example:

```dml

Ceci est un block d'une ligne

Ceci est un deuxième block.
Il s'étend sur trois ligne.
Car il n'y a pas de ligne vide entre deux.

--- bloc

Ceci est un bloc ayant le nom bloc.

Même si il y a des lignes vides, nous sommes toujours dans le
même bloc

---

Ici nous sommes dans un nouveau bloc anonyes.


```

### Types de blocs

#### Bloc annonymes

L'algorytme détecte les blocs selon l'ordre suivant:

- Un bloc annonymes dont la première lettre est un caractère compris entre AZ-az
  0-9 est un paragraphe.
- Un bloc annonymes commençant par un ou plusieurs # suivit d'un espace 
  représente un bloc de titre ayant un niveau coresspondant au nombre de dièses.
- Un bloc annonymes commençant par une série de caractère suivi d'un espace
  dont les caractères ont été enregistré par un plugin comme "pattern" de bloc
  est un bloc du même type que le plugin
- Tous autre bloc anonymes est de type paragraphe.


```dml
Un paragraphe

# Bloc titre niveau 1

## Bloc titre niveau 2

### Bloc titre niveau 3

#### Bloc titre niveau 4

##### Bloc titre niveau 5

###### Bloc titre niveau 6

éy! Si le bloc ne correspond à aucun des patterns ci-dessus, comme notre bloc, le
type est paragraphe.
```


