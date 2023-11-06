// The Swift Programming Language
// https://docs.swift.org/swift-book

import Foundation
import DatadogCore
import DatadogTrace

@main
struct Program {
    static func main() async throws {
        Datadog.initialize(
            with: Datadog.Configuration(
                clientToken: "<client token>",
                env: "test",
                service: "<service name>"
            ),
            trackingConsent: .granted
        )

        Trace.enable(with: .init(urlSessionTracking: .init(firstPartyHostsTracing: .trace(hosts: ["localhost"], sampleRate: 100))))
        let delegate = Delegate()
        let session = URLSession(configuration: .default, delegate: delegate, delegateQueue: nil)
        URLSessionInstrumentation.enable(with: .init(delegateClass: Delegate.self))
        let url = URL(string: "http://localhost:8126/httpbin/get")!
        let (data, _) = try await session.data(from: url)

        let response = try JSONDecoder().decode(GetResponse.self, from: data)

        print("Headers:")
        for (key, value) in response.headers {
            print("  \(key): \(value)")
        }
    }

    class Delegate: NSObject, URLSessionDataDelegate {

    }

    struct GetResponse: Codable {
        let args: [String: String]
        let headers: [String: String]
        let url: String
    }
}
