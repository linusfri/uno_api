### This is a simple Api for myself learning rust. This app is tracking uno scores.

### Stand in base directory
1. Add .env with DATABASE_URL=DATABASE_URL=mysql://uno_user:mysql@db:3306/uno (only for development)
2. Run docker compose up, wait for build to finish and for containers to start
3. Run ./scripts/run_migration.sh to run migrations on the database
4. Enjoy
