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

        Trace.enable(with: .init(urlSessionTracking: .init(firstPartyHostsTracing: .trace(hosts: ["httpbin.org"], sampleRate: 100))))
        let delegate1 = Delegate1()
        let session1 = URLSession(configuration: .default, delegate: delegate1, delegateQueue: nil)
        URLSessionInstrumentation.enable(with: .init(delegateClass: Delegate1.self))
        try await makeRequest(session: session1)

        let delegate2 = Delegate2()
        let session2 = URLSession(configuration: .default, delegate: delegate2, delegateQueue: nil)
        URLSessionInstrumentation.enable(with: .init(delegateClass: Delegate2.self))
        try await makeRequest(session: session2)

        try await makeRequest(session: session1)
    }

    static func makeRequest(session: URLSession) async throws {
        let url = URL(string: "https://httpbin.org/get")!
        let (data, _) = try await session.data(from: url)

        let response = try JSONDecoder().decode(GetResponse.self, from: data)

        print("Headers:")
        for (key, value) in response.headers {
            print("  \(key): \(value)")
        }
    }

    class Delegate1: NSObject, URLSessionDataDelegate {

    }
    
    class Delegate2: NSObject, URLSessionDataDelegate {

    }

    struct GetResponse: Codable {
        let args: [String: String]
        let headers: [String: String]
        let url: String
    }
}
