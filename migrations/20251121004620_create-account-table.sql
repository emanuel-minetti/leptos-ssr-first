CREATE TYPE lang AS ENUM ('en', 'de');

CREATE TABLE public.account
(
    id                 uuid default gen_random_uuid() NOT NULL
        CONSTRAINT account_pk
            PRIMARY KEY,
    username           varchar(20)                    NOT NULL,
    pw_hash            varchar(72)                    NOT NULL,
    name               varchar(80)                    NOT NULL,
    preferred_language lang DEFAULT 'de'              NOT NULL
);


CREATE UNIQUE INDEX account_account_name_uindex
    ON public.account (username);
