# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [✔️] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [✔️] Commit: `Create Subscriber model struct.`
    -   [✔️] Commit: `Create Notification model struct.`
    -   [✔️] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [✔️] Commit: `Implement add function in Subscriber repository.`
    -   [✔️] Commit: `Implement list_all function in Subscriber repository.`
    -   [✔️] Commit: `Implement delete function in Subscriber repository.`
    -   [✔️] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [✔️] Commit: `Create Notification service struct skeleton.`
    -   [✔️] Commit: `Implement subscribe function in Notification service.`
    -   [✔️] Commit: `Implement subscribe function in Notification controller.`
    -   [✔️] Commit: `Implement unsubscribe function in Notification service.`
    -   [✔️] Commit: `Implement unsubscribe function in Notification controller.`
    -   [✔️] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [✔️] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [✔️] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [✔️] Commit: `Implement publish function in Program service and Program controller.`
    -   [✔️] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [✔️] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. For BambangShop case we still need an interface. Subscriber interface allows for different types of subscribers that can react differently to notifications. This way, when a product is added, deleted, or promoted, we can simply call the notify method on all subscribers of the relevant product type, and each subscriber will handle the notification in its own way.

2. Using DashMap is necessary in this case because it provides a more efficient way to store and retrieve unique values based on their keys. DashMap allows for constant-time lookups and inserts, which is important when dealing with unique identifiers like id in Program and url in Subscriber. Using a Vec (list) would require iterating over the entire list to find a specific value, resulting in a less efficient solution. Therefore, using DashMap is recommended for this scenario.

3. In terms of design patterns, using DashMap is necessary for thread safety. While the Singleton pattern ensures only one instance of a class, it doesn't inherently provide thread safety. Therefore, DashMap is a good choice for achieving thread safety in this case.

#### Reflection Publisher-2
1. MVC pattern doesn't explicitly include Services and Repositories, including them can lead to a more organized, flexible, and maintainable system.

2. If we only use model, it will cause the model to be bloated with business logic, making it harder to maintain and test. By separating business logic into services, we can keep the model clean and focused on data representation. Repositories are used to abstract the data layer, allowing for easier testing and swapping of data sources.

3. Postman is a useful tool for testing APIs endpoint because it allows for easy creation and execution of HTTP requests. One of my favorite feature in Postman is we can see the response in a pretty format, which makes it easier to read and understand.

#### Reflection Publisher-3

1. We use the push model in this case because we have a function called notify that notfiy the subscribers

2. The advantages of the pull model is that the subscriber can request the notification when they want to, so the subscriber can control when they want to receive the notification. The disadvantages of the pull model is that the subscriber need to request the notification every time they want to receive the notification.

3. if we didn't use the multi-threading in notification process it will cause the notification process to be slow because the notification process will be done sequentially. By using multi-threading, we can do the notification process concurrently, so the notification process will be faster.
