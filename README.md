# test - [![Build Status](https://travis-ci.org/sinitcin/test.svg?branch=master)](https://travis-ci.org/sinitcin/test) [![CircleCI](https://circleci.com/gh/sinitcin/test.svg?style=svg)](https://circleci.com/gh/sinitcin/test)
--
## План выполнения:
1. ~~Возможность загружать несколько файлов.~~
2. ~~Возможность принимать multipart/form-data запросы.~~
3. ~~Возможность принимать JSON запросы с BASE64 закодированными изображениями.~~
4. ~~Возможность загружать изображения по заданному URL (изображение размещено где-то в интернете).~~
5. ~~Создание квадратного превью изображения размером 100px на 100px.~~
6. ~~Корректное завершение приложения при получении сигнала ОС (graceful shutdown).~~ Только средствами Docker, если заменить сигнал.
7. Dockerfile и docker-compose.yml, которые позволяют поднять приложение единой docker-compose up командой.
8. Модульные тесты, функциональные тесты, CI интеграция (~~Travis CI~~, Circle CI, другие).
9. ~~Использование отдельной внешней библиотеки обработки изображений (к примеру, OpenCV) для демонстрации FFI.~~
