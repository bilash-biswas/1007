(let [num1 (do (Integer/parseInt (read-line)))
     num2 (do (Integer/parseInt (read-line)))
     num3 (do (Integer/parseInt (read-line)))
     num4 (do (Integer/parseInt (read-line)))] 
  (println "DIFERENCA =" (- (* num1 num2) (* num3 num4))))
