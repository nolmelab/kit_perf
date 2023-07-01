# sql perf 

SQLx를 사용하고 json에서 설정을 읽어서 DB 테스트를 진행한다. 
일련의 명령어를 실행하고 일부 명령은 반복한다. 

```json 
{
    "db" : {
        "conn" : "postgress://postgress"
    },

    "flow" : {
        "setup" : [
            { 
                "query" : "

                " 
            },
            {
                "query" : ""
            }
        ],
        "repeat" : [
            {
                "query" : ""
            },
            {
                "query" : ""
            }
        ]
    }
}
```


