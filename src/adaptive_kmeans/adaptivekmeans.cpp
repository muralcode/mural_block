//Copyright (c) 2024 Arithoptix Pty Ltd.
#include "Adaptivekmeans.h"

// Constructor
AdaptiveKMeans::AdaptiveKMeans(const std::vector<Point>& data, int k) : data_(data), k_(k) {
    featureScaling(); // This is to Scale features upon initialization
}

// Function to calculate Euclidean or Mahalanobis distance between two points
double AdaptiveKMeans::calculateDistance(const Point& p1, const Point& p2, bool useMahalanobis) {
    if (useMahalanobis) {
        Eigen::VectorXd diff(p1.coords.size());
        for (size_t i = 0; i < p1.coords.size(); ++i) {
            diff(i) = p1.coords[i] - p2.coords[i];
        }

        Eigen::MatrixXd covarianceMatrix = calculateCovarianceMatrix();
        Eigen::MatrixXd invCov = covarianceMatrix.inverse();
        double distance = std::sqrt(diff.transpose() * invCov * diff);
        return distance;
    } else {
        double distance = 0.0;
        for (size_t i = 0; i < p1.coords.size(); ++i) {
            distance += std::pow(p1.coords[i] - p2.coords[i], 2);
        }
        // for (size_t i = 0; i < p1.length; ++i) {
        //     distance += std::pow(p1.coords[i] - p2.coords[i], 2);
        // }
        return std::sqrt(distance);
    }
}

// Function to calculate the covariance matrix for Mahalanobis distance
Eigen::MatrixXd AdaptiveKMeans::calculateCovarianceMatrix() {
    if (data_.empty()) {
        // Handle case where data_ is empty
        return Eigen::MatrixXd::Zero(0, 0);
    }

    size_t dim = data_[0].coords.size();
    Eigen::MatrixXd dataMatrix(data_.size(), dim);

    for (size_t i = 0; i < data_.size(); ++i) {
        for (size_t j = 0; j < dim; ++j) {
            dataMatrix(i, j) = data_[i].coords[j];
        }
    }
    Eigen::MatrixXd covarianceMatrix = (dataMatrix.transpose() * dataMatrix) / static_cast<double>(data_.size() - 1);
    return covarianceMatrix;
}

// Function to initialize centroids using k-means++ initialization
std::vector<Point> AdaptiveKMeans::initializeCentroids(const std::vector<Point>& data, int k) {
    std::vector<Point> centroids;
    centroids.reserve(k);

    // Choose the first centroid randomly from data
    centroids.push_back(data[std::rand() % data.size()]);

    // Choose subsequent centroids using k-means++ initialization
    while (centroids.size() < static_cast<size_t>(k)) {
        std::vector<double> distances(data.size(), std::numeric_limits<double>::max());
        double totalDistance = 0.0;

        // Calculate distance from each point to nearest centroid
        for (size_t i = 0; i < data.size(); ++i) {
            for (const auto& centroid : centroids) {
                double dist = calculateDistance(data[i], centroid, false);
                distances[i] = std::min(distances[i], dist);
            }
            totalDistance += distances[i];
        }

        // Select new centroid based on weighted probability
        double targetDistance = std::rand() * totalDistance / RAND_MAX;
        double cumulativeDistance = 0.0;
        size_t newIndex = 0;
        while (cumulativeDistance <= targetDistance) {
            cumulativeDistance += distances[newIndex++];
        }
        centroids.push_back(data[newIndex - 1]);
    }

    return centroids;
}

// Function to handle outliers by robustly initializing centroids
std::vector<Point> AdaptiveKMeans::initializeCentroidsRobust(const std::vector<Point>& data, int k, int numInitializationAttempts) {
    std::vector<Point> bestCentroids;
    double minInertia = std::numeric_limits<double>::max();

    // Try multiple initializations and select the best one based on inertia
    for (int attempt = 0; attempt < numInitializationAttempts; ++attempt) {
        std::vector<Point> centroids = initializeCentroids(data, k);

        // Calculate inertia (within-cluster sum of squares)
        double inertia = 0.0;
        std::vector<double> clusterSizes(k, 0);

        for (const auto& point : data) {
            double minDistance = std::numeric_limits<double>::max();
            int closestCluster = -1;
            for (int j = 0; j < k_; ++j) {
                double dist = calculateDistance(point, centroids[j], false);
                if (dist < minDistance) {
                    minDistance = dist;
                    closestCluster = j;
                }
            }
            inertia += minDistance * minDistance;
            clusterSizes[closestCluster]++;
        }

        // Store centroids and inertia if it's the best so far
        if (inertia < minInertia) {
            minInertia = inertia;
            bestCentroids = centroids;
        }
    }

    return bestCentroids;
}

// Function to perform feature scaling (normalization)
void AdaptiveKMeans::featureScaling() {
    size_t dim = data_[0].coords.size();
    std::vector<double> minValues(dim, std::numeric_limits<double>::max());
    std::vector<double> maxValues(dim, std::numeric_limits<double>::min());

    // Find min and max values for each dimension
    for (const auto& point : data_) {
        for (size_t i = 0; i < dim; ++i) {
            minValues[i] = std::min(minValues[i], point.coords[i]);
            maxValues[i] = std::max(maxValues[i], point.coords[i]);
        }
    }

    // Normalize data
    for (auto& point : data_) {
        for (size_t i = 0; i < dim; ++i) {
            if (maxValues[i] != minValues[i]) {
                point.coords[i] = (point.coords[i] - minValues[i]) / (maxValues[i] - minValues[i]);
            }
        }
    }
}

// Function to perform K-means clustering
std::vector<int> AdaptiveKMeans::cluster(int maxIterations, bool useMahalanobis) {
    // Init centroids robustly
    centroids_ = initializeCentroidsRobust(data_, k_, 3); // we attempt 3 initialization attempts

    std::vector<int> clusterAssignments(data_.size());
    std::vector<int> clusterSizes(k_, 0);

    int iteration = 0;
    while (iteration < maxIterations) {
        bool centroidsChanged = false;

        // Assign each point to the nearest centroid
        for (size_t i = 0; i < data_.size(); ++i) {
            double minDistance = std::numeric_limits<double>::max();
            int closestCluster = -1;
            for (int j = 0; j < k_; ++j) {
                double dist = calculateDistance(data_[i], centroids_[j], useMahalanobis);
                if (dist < minDistance) {
                    minDistance = dist;
                    closestCluster = j;
                }
            }
            if (clusterAssignments[i] != closestCluster) {
                clusterAssignments[i] = closestCluster;
                centroidsChanged = true;
            }
            clusterSizes[closestCluster]++;
        }

        // Update centroids
        if (!centroidsChanged) {
            break; // Convergence criteria
        }

        for (int j = 0; j < k_; ++j) {
            Point newCentroid;
            newCentroid.coords.resize(data_[0].coords.size(), 0.0);
            for (size_t i = 0; i < data_.size(); ++i) {
                if (clusterAssignments[i] == j) {
                    for (size_t d = 0; d < data_[i].coords.size(); ++d) {
                        newCentroid.coords[d] += data_[i].coords[d];
                    }
                }
            }
            if (clusterSizes[j] > 0) {
                for (size_t d = 0; d < newCentroid.coords.size(); ++d) {
                    newCentroid.coords[d] /= clusterSizes[j];
                }
            }
            centroids_[j] = newCentroid;
        }

        iteration++;
    }

    return clusterAssignments;
}

// Function to perform ensemble clustering
std::vector<int> AdaptiveKMeans::ensembleClustering(const std::vector<std::vector<int>>& clusteringResults) {
    size_t numPoints = clusteringResults[0].size();
    size_t numClusters = clusteringResults.size();
    std::vector<int> finalAssignments(numPoints, 0);

    for (size_t i = 0; i < numPoints; ++i) {
        std::vector<int> clusterVotes(numClusters, 0);
        for (const auto& result : clusteringResults) {
            clusterVotes[result[i]]++;
        }
        finalAssignments[i] = std::distance(clusterVotes.begin(), std::max_element(clusterVotes.begin(), clusterVotes.end()));
    }

    return finalAssignments;
}

// Extern "C" function for adaptive k-means clustering
extern "C" {
    void adaptive_kmeans(const Point* data, size_t len, int k, int max_iterations, bool use_mahalanobis, KMeansResult* result) {
        std::vector<Point> data_vec(data, data + len);
        AdaptiveKMeans kmeans(data_vec, k);
        std::vector<int> assignments = kmeans.cluster(max_iterations, use_mahalanobis);
        result->assignments = assignments;
    }
}
