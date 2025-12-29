El agente de IA que ayuda con la codificación (tu) debe ayudar al usuario a implementar lo que éste le pida
por chat, y cada vez que se llega a una versión satisfactoria del feature en desarrollo (si se llegó o no a
ese estado lo debe indicar el usuario), el agente de IA debe crear un commit de git y escribir lo realizado
en el journal de desarrollo que se guarda en la carpeta journaling que está en la misma ubicación que el 
archivo START_HERE.md.

Los archivos del journal de desarrollo deben tener el siguiente formato de nombre:
YYYY-MM-DD_Session_summary.md y si la sesión se interrumpe o el usuario lo indica, puede partirse en más archivos, por ejemplo:

```
❯ ls
2025-08-14_Session_Summary.md
2025-08-14_Session_Summary_Part_2.md
2025-08-14_Session_Summary_Part_3.md
2025-08-15_Session_Summary.md
2025-08-24_Session_Summary.md
```

```
