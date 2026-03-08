ALTER TABLE public.session
    DROP CONSTRAINT session_account_id_fk;

ALTER TABLE public.session
    ADD CONSTRAINT session_account_id_fk
        FOREIGN KEY (account_id) REFERENCES public.account
            ON DELETE CASCADE;
