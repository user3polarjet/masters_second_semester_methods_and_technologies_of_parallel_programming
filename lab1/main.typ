#set text(font: "Times New Roman", size: 14pt)
#set page(
  paper: "a4",
  margin: (top: 1.5cm, bottom: 1.5cm, left: 2.5cm, right: 1.5cm)
)
#align(center)[
#image("kpi.png", width: 75%)

Міністерство освіти і науки України

Національний технічний університет України

“Київський політехнічний інститут імені Ігоря Сікорського”

Факультет інформатики та обчислювальної техніки

Кафедра інформатики та програмної інженерії

#align(horizon)[
  #text(size: 18pt)[
    *Лабораторна робота №1*

    Методи і технології паралельного програмування
    ]

    Тема: Засоби створення та керування потоками в паралельних мультипоточних програмах
  ]

  #columns(2, gutter: 8pt)[
    #align(left)[
      Виконав:

      студент групи ІП-51мн

      Панченко С. В.
    ]

    #colbreak()

    #align(right)[
      Перевірив:

      Лесик В. О.
    ]
  ]
]

#align(center + bottom)[
  Київ 2026
]

#set page(numbering: "1")
#show outline: it => {
  show heading: set align(center)
  show heading: set text(weight: "regular")
  it
}
#outline(title: upper([Зміст]))

#pagebreak()

#set heading(numbering: "1.")

= Вступ
Це початок вашої роботи...

= Основна частина
== Аналіз результатів
Тут будуть ваші графіки та таблиці зі швидкістю (speedup).
