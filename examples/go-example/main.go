package main

import (
	"context"
	"errors"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/codes"
	"go.opentelemetry.io/otel/trace"
	ddotel "gopkg.in/DataDog/dd-trace-go.v1/ddtrace/opentelemetry"
)

func operationThatCouldFail() (string, error) {
	return "", errors.New("empty name")
}

func main() {
	provider := ddotel.NewTracerProvider()
	defer provider.Shutdown()

	otel.SetTracerProvider(provider)

	tracer := otel.Tracer("")
	_, span := tracer.Start(
		context.Background(),
		"span_name",
		trace.WithAttributes(attribute.String("hello", "world")))
	span.SetAttributes(attribute.Bool("isTrue", true), attribute.String("stringAttr", "hi!"))
	var myKey = attribute.Key("myCoolAttribute")
	span.SetAttributes(myKey.String("a value"))
	span.AddEvent("Acquiring lock")
	span.AddEvent("Got lock, doing work...")
	span.AddEvent("Unlocking")
	span.AddEvent("Cancelled wait due to external signal", trace.WithAttributes(attribute.Int("pid", 4328), attribute.String("signal", "SIGHUP")))
	span.SetStatus(codes.Error, "operationThatCouldFail failed")
	_, err := operationThatCouldFail()
	span.RecordError(err)
	span.RecordError(err)

	span.End()
}
