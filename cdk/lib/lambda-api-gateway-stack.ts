import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'cargo-lambda-cdk';
import * as path from 'path';
import { RestApi, LambdaIntegration, Cors } from 'aws-cdk-lib/aws-apigateway';
import { Architecture } from 'aws-cdk-lib/aws-lambda';

export class InfrastructureStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    // package-name is the name of the Rust pacakge name from the Cargo.toml file
    const fn = new RustFunction(this, 'sample-api', {
      manifestPath: path.join(__dirname, '..', '..'),
      functionName: 'hello-world-rust',
    });

    // Create an API Gateway resource for each of the CRUD operations
    const api = new RestApi(this, 'api', {
      restApiName: 'Rust API',

    });

    // adds /api to the API Gateway
    const routes = api.root.addResource('api');
    // add /test routes for lambda with CORS
    const trackUsage = routes.addResource('test');
    trackUsage.addCorsPreflight({ allowOrigins: ["*"], allowMethods: ["POST"], allowHeaders: ["*"], allowCredentials: true });
    trackUsage.addMethod('POST', new LambdaIntegration(fn));

  }
}
