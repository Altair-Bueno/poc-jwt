FROM openjdk:17-jdk-alpine as builder
ADD . /source
WORKDIR /source
RUN ./mvnw package -DskipTests
RUN mv /source/target/*.jar /source/target/app.jar

FROM openjdk:17-jdk-alpine
RUN addgroup -S spring && adduser -S spring -G spring
USER spring:spring
COPY --from=builder /source/target/app.jar /app.jar
ENV SPRING_PROFILES_ACTIVE=prod
ENTRYPOINT ["java","-jar","/app.jar"]