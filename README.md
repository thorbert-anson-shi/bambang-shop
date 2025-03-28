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
    | variable | type | description |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)

- [✅] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
- **STAGE 1: Implement models and repositories**
  - [✅] Commit: `Create Subscriber model struct.`
  - [✅] Commit: `Create Notification model struct.`
  - [✅] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
  - [✅] Commit: `Implement add function in Subscriber repository.`
  - [✅] Commit: `Implement list_all function in Subscriber repository.`
  - [✅] Commit: `Implement delete function in Subscriber repository.`
  - [✅] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
- **STAGE 2: Implement services and controllers**
  - [✅] Commit: `Create Notification service struct skeleton.`
  - [✅] Commit: `Implement subscribe function in Notification service.`
  - [✅] Commit: `Implement subscribe function in Notification controller.`
  - [✅] Commit: `Implement unsubscribe function in Notification service.`
  - [✅] Commit: `Implement unsubscribe function in Notification controller.`
  - [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
- **STAGE 3: Implement notification mechanism**
  - [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
  - [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
  - [ ] Commit: `Implement publish function in Program service and Program controller.`
  - [ ] Commit: `Edit Product service methods to call notify after create/delete.`
  - [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections

This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. At this point, I can't really see hwo the Observer pattern is going to be used. However, assuming that the subscriber is simply listening for all updates from a particular product, then the subscriber `struct` seems sufficient. The URL + name abstraction is enough for the publisher to identify which subscribers to send the updates to, and more importantly where to send the updates.

2. In this case, using a `Vec` is enough, but iterating through an entire `Vec` to ensure uniqueness will take a lot more time (O(1) vs O(n)) than using a `DashMap` (a `HashMap` but with built in concurrency).

3. Reading from the documentation of the `DashMap`, it is at a high level a `RwLock` wrapper around a `HashMap` instance. This allows concurrent use of the same `HashMap` instance in memory. The Singleton pattern aims to allow access to the same object instance in memory. If we were to use a plain `HashMap` instead of a `DashMap`, then multiple threads could be accessing the same `HashMap` instance in memory. When this happens, the usual concurrency issues, such as race conditions, lost updates, etc., will still affect this `HashMap` instance.

   In conclusion, the singleton pattern doesn't solve the intrinsic issue of using a `HashMap` concurrently. The issue is only solved properly by adding concurrency control, such as a mutex lock to the `HashMap`, which is essentially what the `DashMap` does.

#### Reflection Publisher-2

1. The reason why the model is separated into "service" and "repository" is done because a traditional Model in the MVC architecture violates the Single Responsibility Principle (SRP) by handling both business logic and data management.

   After the refactoring, the service component handles strictly business logic, while the repository handles strictly data storage and management.

   This allows for loose coupling between the business logic and the data storage logic, which in turn allows for future development

   This also enforces the Open Closed Principle by preventing the developer from making changes to the business logic and data management logic at the same time, thus minimizing the risk of feature regression as new updates roll in.

2. I could imagine the logic to be more compact and optimized, as all the code can now communicate between one another more granularly. However, it could be harder to enforce separation of concerns, causing the data management logic to mix with business logic.

   Firstly, this would make unit testing more difficult, as it isn't clear which "unit" we're testing on. I would also imagine that changes to the underlying data structure would require sweeping changes to the way data is accessed across the whole model.

3. I have used Postman to test API endpoints in order to test them more extensively than a browser can. Right now, it can be used to make requests to the application in a repeatable and user friendly way. For example, instead of coming up with HTTP requests from scratch, Postman can be used to make POST and PUT requests to send to the application.

   In the future, I plan to use Postman to share API testing endpoints with colleagues in order to make sure we're all on the same page when it comes to the API structure and payload to provide.

#### Reflection Publisher-3
