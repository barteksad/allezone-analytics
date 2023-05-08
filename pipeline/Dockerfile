FROM maven:3.6.0-jdk-11-slim AS build
COPY . .
RUN mvn clean package
ENTRYPOINT ["mvn", "exec:java", "-Dexec.mainClass=aggregates.Main"]