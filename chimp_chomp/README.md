# CHiMP Worker

This worker steals jobs from a RabbitMQ queue, retrieves images, performs batch inference on them using the CHiMP neural network and returns results on another RabbitMQ queue. The worker is intended to be deployed as a autoscaled to zero service.
