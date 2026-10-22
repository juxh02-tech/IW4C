# IW4C sur iPhone depuis Windows 11

## Compilation

1. Ouvrir Actions dans le dépôt juxh02-tech/IW4C.
2. Activer les workflows du fork si GitHub le demande.
3. Choisir « IW4C iPhone », puis « Run workflow », branche main.
4. Laisser le tag vide pour produire un artefact sans publier de release.
5. Attendre la réussite du travail ipa. Télécharger IW4C-iPhone dans Artifacts.
6. Extraire le ZIP : IW4C.ipa est l’application non signée.

## Installation et première partie

1. Installer Sideloadly depuis https://sideloadly.io/ sur Windows.
2. Brancher l’iPhone en USB et accepter la confiance sur le téléphone.
3. Sélectionner IW4C.ipa dans Sideloadly et signer avec son compte Apple.
   Saisir les identifiants uniquement dans les outils Apple/Sideloadly concernés.
4. Suivre les indications iOS pour autoriser l’application et le mode développeur.
5. Ouvrir IW4C une fois, puis fermer l’application.
6. Copier sa propre installation PC de MW2 dans Fichiers > Sur mon iPhone > IW4C > Games.
   Le dossier de jeu doit contenir zone. Aucun fichier MW2 n’est fourni ici.
7. Connecter une manette Bluetooth. Le tactile sert actuellement aux menus.
8. Relancer IW4C et essayer Rust avec des bots.

## État de cette version

Le nom iOS est IW4C et l’identifiant est com.juxh02tech.iw4c.
Le binaire et les options internes conservent certains noms IW4L/IW4 Pocket.
Les crédits et les licences d’origine sont conservés.
La signature peut nécessiter des renouvellements selon le compte utilisé.
La compilation vérifie le paquet ; elle ne prouve pas la stabilité sur téléphone.
Le menu Cupidon, le jeu entièrement tactile et la connexion IW4x ne sont pas implémentés.
Ne pas désinstaller une application contenant les ressources sans sauvegarde.
