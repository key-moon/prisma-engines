use indoc::indoc;
use introspection_engine_tests::test_api::*;

// referentialIntegrity = "prisma" loses track of the relation policy ("prisma"), but preserves @relations.
#[test_connector(tags(Mssql))]
async fn referential_integrity_prisma(api: &TestApi) -> TestResult {
    let init = formatdoc! {r#"
        CREATE TABLE [dbo].[Foo] (
            [id] INT NOT NULL,
            [bar_id] INT NOT NULL,
            CONSTRAINT [Foo_pkey] PRIMARY KEY CLUSTERED ([id]),
            CONSTRAINT [Foo_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
        );
        
        CREATE TABLE [dbo].[Bar] (
            [id] INT NOT NULL,
            CONSTRAINT [Bar_pkey] PRIMARY KEY CLUSTERED ([id])
        );
    "#};

    api.raw_cmd(&init).await;

    let input = indoc! {r#"
        generator client {
            provider        = "prisma-client-js"
            previewFeatures = ["referentialIntegrity"]
        }

        datasource db {
            provider             = "sqlserver"
            url                  = env("TEST_DATABASE_URL")
            referentialIntegrity = "prisma"
        }

        model Foo {
            id     Int @id
            bar    Bar @relation(fields: [bar_id], references: [id])
            bar_id Int @unique
        }

        model Bar {
            id  Int  @id
            foo Foo?
        }
    "#};

    let expected = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["referentialIntegrity"]
        }

        datasource db {
          provider = "sqlserver"
          url      = env("TEST_DATABASE_URL")
        }

        model Foo {
          id     Int @id
          bar    Bar @relation(fields: [bar_id], references: [id])
          bar_id Int @unique
        }

        model Bar {
          id  Int  @id
          foo Foo?
        }
    "#]];

    let result = api.re_introspect_config(input).await?;
    expected.assert_eq(&result);

    Ok(())
}

// referentialIntegrity = "prisma" loses track of the relation policy ("foreignKeys"), but preserves @relations, which are moved to the bottom.
#[test_connector(tags(Mssql))]
async fn referential_integrity_foreign_keys(api: &TestApi) -> TestResult {
    let init = formatdoc! {r#"
        CREATE TABLE [dbo].[Foo] (
            [id] INT NOT NULL,
            [bar_id] INT NOT NULL,
            CONSTRAINT [Foo_pkey] PRIMARY KEY CLUSTERED ([id]),
            CONSTRAINT [Foo_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
        );
        
        CREATE TABLE [dbo].[Bar] (
            [id] INT NOT NULL,
            CONSTRAINT [Bar_pkey] PRIMARY KEY CLUSTERED ([id])
        );
        
        ALTER TABLE [dbo].[Foo] ADD CONSTRAINT [Foo_bar_id_fkey] FOREIGN KEY ([bar_id]) REFERENCES [dbo].[Bar]([id]) ON DELETE NO ACTION ON UPDATE CASCADE;
    "#};

    api.raw_cmd(&init).await;

    let input = indoc! {r#"
        generator client {
            provider        = "prisma-client-js"
            previewFeatures = ["referentialIntegrity"]
        }

        datasource db {
            provider             = "sqlserver"
            url                  = env("TEST_DATABASE_URL")
            referentialIntegrity = "foreignKeys"
        }

        model Foo {
            id     Int @id
            bar    Bar @relation(fields: [bar_id], references: [id])
            bar_id Int @unique
        }

        model Bar {
            id  Int  @id
            foo Foo?
        }
    "#};

    let expected = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["referentialIntegrity"]
        }

        datasource db {
          provider = "sqlserver"
          url      = env("TEST_DATABASE_URL")
        }

        model Foo {
          id     Int @id
          bar_id Int @unique
          bar    Bar @relation(fields: [bar_id], references: [id])
        }

        model Bar {
          id  Int  @id
          foo Foo?
        }
    "#]];

    let result = api.re_introspect_config(input).await?;
    expected.assert_eq(&result);

    Ok(())
}

// relationMode = "prisma" preserves the relation policy ("prisma") as well as @relations.
#[test_connector(tags(Mssql))]
async fn relation_mode_prisma(api: &TestApi) -> TestResult {
    let init = formatdoc! {r#"
        CREATE TABLE [dbo].[Foo] (
            [id] INT NOT NULL,
            [bar_id] INT NOT NULL,
            CONSTRAINT [Foo_pkey] PRIMARY KEY CLUSTERED ([id]),
            CONSTRAINT [Foo_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
        );
        
        CREATE TABLE [dbo].[Bar] (
            [id] INT NOT NULL,
            CONSTRAINT [Bar_pkey] PRIMARY KEY CLUSTERED ([id])
        );
    "#};

    api.raw_cmd(&init).await;

    let input = indoc! {r#"
        generator client {
            provider        = "prisma-client-js"
            previewFeatures = ["referentialIntegrity"]
        }

        datasource db {
            provider     = "sqlserver"
            url          = env("TEST_DATABASE_URL")
            relationMode = "prisma"
        }

        model Foo {
            id     Int @id
            bar    Bar @relation(fields: [bar_id], references: [id])
            bar_id Int @unique
        }

        model Bar {
            id  Int  @id
            foo Foo?
        }
    "#};

    let expected = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["referentialIntegrity"]
        }

        datasource db {
          provider     = "sqlserver"
          url          = env("TEST_DATABASE_URL")
          relationMode = "prisma"
        }

        model Foo {
          id     Int @id
          bar    Bar @relation(fields: [bar_id], references: [id])
          bar_id Int @unique
        }

        model Bar {
          id  Int  @id
          foo Foo?
        }
    "#]];

    let result = api.re_introspect_config(input).await?;
    expected.assert_eq(&result);

    Ok(())
}

// relationMode = "foreignKeys" preserves the relation policy ("foreignKeys") as well as @relations, which are moved to the bottom.
#[test_connector(tags(Mssql))]
async fn relation_mode_foreign_keys(api: &TestApi) -> TestResult {
    let init = formatdoc! {r#"
        CREATE TABLE [dbo].[Foo] (
            [id] INT NOT NULL,
            [bar_id] INT NOT NULL,
            CONSTRAINT [Foo_pkey] PRIMARY KEY CLUSTERED ([id]),
            CONSTRAINT [Foo_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
        );
        
        CREATE TABLE [dbo].[Bar] (
            [id] INT NOT NULL,
            CONSTRAINT [Bar_pkey] PRIMARY KEY CLUSTERED ([id])
        );
        
        ALTER TABLE [dbo].[Foo] ADD CONSTRAINT [Foo_bar_id_fkey] FOREIGN KEY ([bar_id]) REFERENCES [dbo].[Bar]([id]) ON DELETE NO ACTION ON UPDATE CASCADE;
    "#};

    api.raw_cmd(&init).await;

    let input = indoc! {r#"
        generator client {
            provider        = "prisma-client-js"
            previewFeatures = ["referentialIntegrity"]
        }

        datasource db {
            provider     = "sqlserver"
            url          = env("TEST_DATABASE_URL")
            relationMode = "foreignKeys"
        }

        model Foo {
            id     Int @id
            bar    Bar @relation(fields: [bar_id], references: [id])
            bar_id Int @unique
        }

        model Bar {
            id  Int  @id
            foo Foo?
        }
    "#};

    let expected = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["referentialIntegrity"]
        }

        datasource db {
          provider     = "sqlserver"
          url          = env("TEST_DATABASE_URL")
          relationMode = "foreignKeys"
        }

        model Foo {
          id     Int @id
          bar_id Int @unique
          bar    Bar @relation(fields: [bar_id], references: [id])
        }

        model Bar {
          id  Int  @id
          foo Foo?
        }
    "#]];

    let result = api.re_introspect_config(input).await?;
    expected.assert_eq(&result);

    Ok(())
}

// @relations are moved to the bottom of the model even when no referentialIntegrity/relationMode is used.
#[test_connector(tags(Mssql))]
async fn no_relation_mode(api: &TestApi) -> TestResult {
    let init = formatdoc! {r#"
        CREATE TABLE [dbo].[Foo] (
            [id] INT NOT NULL,
            [bar_id] INT NOT NULL,
            CONSTRAINT [Foo_pkey] PRIMARY KEY CLUSTERED ([id]),
            CONSTRAINT [Foo_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
        );
        
        CREATE TABLE [dbo].[Bar] (
            [id] INT NOT NULL,
            CONSTRAINT [Bar_pkey] PRIMARY KEY CLUSTERED ([id])
        );
        
        ALTER TABLE [dbo].[Foo] ADD CONSTRAINT [Foo_bar_id_fkey] FOREIGN KEY ([bar_id]) REFERENCES [dbo].[Bar]([id]) ON DELETE NO ACTION ON UPDATE CASCADE;
    "#};

    api.raw_cmd(&init).await;

    let input = indoc! {r#"
        datasource db {
            provider = "sqlserver"
            url      = env("TEST_DATABASE_URL")
        }

        model Foo {
            id     Int @id
            bar    Bar @relation(fields: [bar_id], references: [id])
            bar_id Int @unique
        }

        model Bar {
            id  Int  @id
            foo Foo?
        }
    "#};

    let expected = expect![[r#"
        datasource db {
          provider = "sqlserver"
          url      = env("TEST_DATABASE_URL")
        }

        model Foo {
          id     Int @id
          bar_id Int @unique
          bar    Bar @relation(fields: [bar_id], references: [id])
        }

        model Bar {
          id  Int  @id
          foo Foo?
        }
    "#]];

    let result = api.re_introspect_config(input).await?;
    expected.assert_eq(&result);

    Ok(())
}

// @@map
mod at_at_map {
    use indoc::indoc;
    use introspection_engine_tests::test_api::*;

    // referentialIntegrity = "prisma" with @@map loses track of the relation policy ("prisma") and of @relations.
    #[test_connector(tags(Mssql))]
    async fn referential_integrity_prisma(api: &TestApi) -> TestResult {
        let init = formatdoc! {r#"
            CREATE TABLE [dbo].[foo_table] (
                [id] INT NOT NULL,
                [bar_id] INT NOT NULL,
                CONSTRAINT [foo_table_pkey] PRIMARY KEY CLUSTERED ([id]),
                CONSTRAINT [foo_table_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
            );
            
            CREATE TABLE [dbo].[bar_table] (
                [id] INT NOT NULL,
                CONSTRAINT [bar_table_pkey] PRIMARY KEY CLUSTERED ([id])
            );
        "#};

        api.raw_cmd(&init).await;

        let input = indoc! {r#"
            generator client {
                provider        = "prisma-client-js"
                previewFeatures = ["referentialIntegrity"]
            }

            datasource db {
                provider             = "sqlserver"
                url                  = env("TEST_DATABASE_URL")
                referentialIntegrity = "prisma"
            }

            model Foo {
                id     Int @id
                bar    Bar @relation(fields: [bar_id], references: [id])
                bar_id Int @unique

                @@map("foo_table")
            }

            model Bar {
                id  Int  @id
                foo Foo?

                @@map("bar_table")
            }
        "#};

        let expected = expect![[r#"
            generator client {
              provider        = "prisma-client-js"
              previewFeatures = ["referentialIntegrity"]
            }

            datasource db {
              provider = "sqlserver"
              url      = env("TEST_DATABASE_URL")
            }

            model Foo {
              id     Int @id
              bar_id Int @unique

              @@map("foo_table")
            }

            model Bar {
              id Int @id

              @@map("bar_table")
            }
        "#]];

        let result = api.re_introspect_config(input).await?;
        expected.assert_eq(&result);

        Ok(())
    }

    // referentialIntegrity = "foreignKeys" with @@map loses track of the relation policy ("foreignKeys"), but preserves @relations, which are moved to the bottom.
    #[test_connector(tags(Mssql))]
    async fn referential_integrity_foreign_keys(api: &TestApi) -> TestResult {
        let init = formatdoc! {r#"
            CREATE TABLE [dbo].[foo_table] (
                [id] INT NOT NULL,
                [bar_id] INT NOT NULL,
                CONSTRAINT [foo_table_pkey] PRIMARY KEY CLUSTERED ([id]),
                CONSTRAINT [foo_table_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
            );
            
            CREATE TABLE [dbo].[bar_table] (
                [id] INT NOT NULL,
                CONSTRAINT [bar_table_pkey] PRIMARY KEY CLUSTERED ([id])
            );
            
            ALTER TABLE [dbo].[foo_table] ADD CONSTRAINT [foo_table_bar_id_fkey] FOREIGN KEY ([bar_id]) REFERENCES [dbo].[bar_table]([id]) ON DELETE NO ACTION ON UPDATE CASCADE;
        "#};

        api.raw_cmd(&init).await;

        let input = indoc! {r#"
            generator client {
                provider        = "prisma-client-js"
                previewFeatures = ["referentialIntegrity"]
            }

            datasource db {
                provider             = "sqlserver"
                url                  = env("TEST_DATABASE_URL")
                referentialIntegrity = "foreignKeys"
            }

            model Foo {
                id     Int @id
                bar    Bar @relation(fields: [bar_id], references: [id])
                bar_id Int @unique

                @@map("foo_table")
            }

            model Bar {
                id  Int  @id
                foo Foo?

                @@map("bar_table")
            }
        "#};

        let expected = expect![[r#"
            generator client {
              provider        = "prisma-client-js"
              previewFeatures = ["referentialIntegrity"]
            }

            datasource db {
              provider = "sqlserver"
              url      = env("TEST_DATABASE_URL")
            }

            model Foo {
              id     Int @id
              bar_id Int @unique
              bar    Bar @relation(fields: [bar_id], references: [id])

              @@map("foo_table")
            }

            model Bar {
              id  Int  @id
              foo Foo?

              @@map("bar_table")
            }
        "#]];

        let result = api.re_introspect_config(input).await?;
        expected.assert_eq(&result);

        Ok(())
    }

    // relationMode = "prisma" with @@map preserves the relation policy ("prisma"), but loses track of @relations.
    #[test_connector(tags(Mssql))]
    async fn relation_mode_prisma(api: &TestApi) -> TestResult {
        let init = formatdoc! {r#"
            CREATE TABLE [dbo].[foo_table] (
                [id] INT NOT NULL,
                [bar_id] INT NOT NULL,
                CONSTRAINT [foo_table_pkey] PRIMARY KEY CLUSTERED ([id]),
                CONSTRAINT [foo_table_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
            );
            
            CREATE TABLE [dbo].[bar_table] (
                [id] INT NOT NULL,
                CONSTRAINT [bar_table_pkey] PRIMARY KEY CLUSTERED ([id])
            );
        "#};

        api.raw_cmd(&init).await;

        let input = indoc! {r#"
            generator client {
                provider        = "prisma-client-js"
                previewFeatures = ["referentialIntegrity"]
            }

            datasource db {
                provider     = "sqlserver"
                url          = env("TEST_DATABASE_URL")
                relationMode = "prisma"
            }

            model Foo {
                id     Int @id
                bar    Bar @relation(fields: [bar_id], references: [id])
                bar_id Int @unique

                @@map("foo_table")
            }

            model Bar {
                id  Int  @id
                foo Foo?

                @@map("bar_table")
            }
        "#};

        let expected = expect![[r#"
            generator client {
              provider        = "prisma-client-js"
              previewFeatures = ["referentialIntegrity"]
            }

            datasource db {
              provider     = "sqlserver"
              url          = env("TEST_DATABASE_URL")
              relationMode = "prisma"
            }

            model Foo {
              id     Int @id
              bar_id Int @unique

              @@map("foo_table")
            }

            model Bar {
              id Int @id

              @@map("bar_table")
            }
        "#]];

        let result = api.re_introspect_config(input).await?;
        expected.assert_eq(&result);

        Ok(())
    }

    // relationMode = "foreignKeys" with @@map preserves the relation policy ("foreignKeys") and @relations, which are moved to the bottom.
    #[test_connector(tags(Mssql))]
    async fn relation_mode_foreign_keys(api: &TestApi) -> TestResult {
        let init = formatdoc! {r#"
            CREATE TABLE [dbo].[foo_table] (
                [id] INT NOT NULL,
                [bar_id] INT NOT NULL,
                CONSTRAINT [foo_table_pkey] PRIMARY KEY CLUSTERED ([id]),
                CONSTRAINT [foo_table_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
            );
            
            CREATE TABLE [dbo].[bar_table] (
                [id] INT NOT NULL,
                CONSTRAINT [bar_table_pkey] PRIMARY KEY CLUSTERED ([id])
            );
            
            ALTER TABLE [dbo].[foo_table] ADD CONSTRAINT [foo_table_bar_id_fkey] FOREIGN KEY ([bar_id]) REFERENCES [dbo].[bar_table]([id]) ON DELETE NO ACTION ON UPDATE CASCADE;
        "#};

        api.raw_cmd(&init).await;

        let input = indoc! {r#"
            generator client {
                provider        = "prisma-client-js"
                previewFeatures = ["referentialIntegrity"]
            }

            datasource db {
                provider     = "sqlserver"
                url          = env("TEST_DATABASE_URL")
                relationMode = "foreignKeys"
            }

            model Foo {
                id     Int @id
                bar    Bar @relation(fields: [bar_id], references: [id])
                bar_id Int @unique

                @@map("foo_table")
            }

            model Bar {
                id  Int  @id
                foo Foo?

                @@map("bar_table")
            }
        "#};

        let expected = expect![[r#"
            generator client {
              provider        = "prisma-client-js"
              previewFeatures = ["referentialIntegrity"]
            }

            datasource db {
              provider     = "sqlserver"
              url          = env("TEST_DATABASE_URL")
              relationMode = "foreignKeys"
            }

            model Foo {
              id     Int @id
              bar_id Int @unique
              bar    Bar @relation(fields: [bar_id], references: [id])

              @@map("foo_table")
            }

            model Bar {
              id  Int  @id
              foo Foo?

              @@map("bar_table")
            }
        "#]];

        let result = api.re_introspect_config(input).await?;
        expected.assert_eq(&result);

        Ok(())
    }

    // @relations are moved to the bottom of the model even when no referentialIntegrity/relationMode is used and @@map is used.
    #[test_connector(tags(Mssql))]
    async fn no_relation(api: &TestApi) -> TestResult {
        let init = formatdoc! {r#"
            CREATE TABLE [dbo].[foo_table] (
                [id] INT NOT NULL,
                [bar_id] INT NOT NULL,
                CONSTRAINT [foo_table_pkey] PRIMARY KEY CLUSTERED ([id]),
                CONSTRAINT [foo_table_bar_id_key] UNIQUE NONCLUSTERED ([bar_id])
            );
            
            CREATE TABLE [dbo].[bar_table] (
                [id] INT NOT NULL,
                CONSTRAINT [bar_table_pkey] PRIMARY KEY CLUSTERED ([id])
            );
            
            ALTER TABLE [dbo].[foo_table] ADD CONSTRAINT [foo_table_bar_id_fkey] FOREIGN KEY ([bar_id]) REFERENCES [dbo].[bar_table]([id]) ON DELETE NO ACTION ON UPDATE CASCADE;
        "#};

        api.raw_cmd(&init).await;

        let input = indoc! {r#"
            datasource db {
                provider = "sqlserver"
                url      = env("TEST_DATABASE_URL")
            }

            model Foo {
                id     Int @id
                bar    Bar @relation(fields: [bar_id], references: [id])
                bar_id Int @unique

                @@map("foo_table")
            }

            model Bar {
                id  Int  @id
                foo Foo?

                @@map("bar_table")
            }
        "#};

        let expected = expect![[r#"
            datasource db {
              provider = "sqlserver"
              url      = env("TEST_DATABASE_URL")
            }

            model Foo {
              id     Int @id
              bar_id Int @unique
              bar    Bar @relation(fields: [bar_id], references: [id])

              @@map("foo_table")
            }

            model Bar {
              id  Int  @id
              foo Foo?

              @@map("bar_table")
            }
        "#]];

        let result = api.re_introspect_config(input).await?;
        expected.assert_eq(&result);

        Ok(())
    }
}
