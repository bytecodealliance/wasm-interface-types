;; parse-fail

(module

  (@interface foo))

(; CHECK-ALL:
unexpected token, expected one of: `type`, `import`, `export`, `func`, `implement`
     --> tests/not-interface2.wast:5:15
      |
    5 |   (@interface foo))
      |               ^
;)
